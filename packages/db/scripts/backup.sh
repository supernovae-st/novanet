#!/usr/bin/env bash
# packages/db/scripts/backup.sh
# Neo4j Docker volume backup/restore utility
# v10.6.0 — Data Persistence Strategy (Option A+B)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
BACKUP_DIR="${PROJECT_ROOT}/backups"
VOLUME_NAME="novanet_neo4j_data"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

usage() {
    cat <<EOF
Neo4j Backup/Restore Utility

Usage: $(basename "$0") <command> [options]

Commands:
    backup              Create a backup of the Neo4j data volume
    restore <file>      Restore from a backup file
    list                List available backups
    export-cypher       Export data as Cypher (requires APOC)

Options:
    --name <name>       Custom backup name (default: timestamp)
    --volume <name>     Docker volume name (default: ${VOLUME_NAME})

Examples:
    $(basename "$0") backup
    $(basename "$0") backup --name before-migration
    $(basename "$0") restore backups/neo4j-20240205-143022.tar.gz
    $(basename "$0") list

EOF
    exit 1
}

log_info() { echo -e "${BLUE}ℹ${NC}  $*"; }
log_success() { echo -e "${GREEN}✓${NC}  $*"; }
log_warn() { echo -e "${YELLOW}⚠${NC}  $*"; }
log_error() { echo -e "${RED}✗${NC}  $*"; }

ensure_backup_dir() {
    mkdir -p "$BACKUP_DIR"
}

check_volume_exists() {
    if ! docker volume inspect "$VOLUME_NAME" &>/dev/null; then
        log_error "Volume '$VOLUME_NAME' not found"
        log_info "Available volumes:"
        docker volume ls --format '  - {{.Name}}' | grep -i neo4j || echo "  (none)"
        exit 1
    fi
}

do_backup() {
    local name="${1:-$TIMESTAMP}"
    local backup_file="${BACKUP_DIR}/neo4j-${name}.tar.gz"

    ensure_backup_dir
    check_volume_exists

    log_info "Backing up volume '$VOLUME_NAME'..."
    log_info "Target: $backup_file"

    # Stop Neo4j if running (for consistent backup)
    if docker ps --format '{{.Names}}' | grep -q neo4j; then
        log_warn "Neo4j container is running. Stopping for consistent backup..."
        docker compose -f "$PROJECT_ROOT/packages/db/docker-compose.yml" stop neo4j
        local was_running=true
    fi

    # Create backup
    docker run --rm \
        -v "${VOLUME_NAME}:/data:ro" \
        -v "${BACKUP_DIR}:/backup" \
        busybox tar czf "/backup/neo4j-${name}.tar.gz" -C /data .

    # Restart if was running
    if [[ "${was_running:-false}" == "true" ]]; then
        log_info "Restarting Neo4j..."
        docker compose -f "$PROJECT_ROOT/packages/db/docker-compose.yml" start neo4j
    fi

    local size=$(du -h "$backup_file" | cut -f1)
    log_success "Backup created: $backup_file ($size)"
}

do_restore() {
    local backup_file="$1"

    if [[ ! -f "$backup_file" ]]; then
        # Try relative to backup dir
        if [[ -f "${BACKUP_DIR}/${backup_file}" ]]; then
            backup_file="${BACKUP_DIR}/${backup_file}"
        else
            log_error "Backup file not found: $backup_file"
            exit 1
        fi
    fi

    log_warn "This will OVERWRITE all data in volume '$VOLUME_NAME'"
    read -p "Are you sure? (yes/no): " confirm
    if [[ "$confirm" != "yes" ]]; then
        log_info "Cancelled"
        exit 0
    fi

    # Stop Neo4j if running
    if docker ps --format '{{.Names}}' | grep -q neo4j; then
        log_info "Stopping Neo4j..."
        docker compose -f "$PROJECT_ROOT/packages/db/docker-compose.yml" stop neo4j
    fi

    log_info "Restoring from: $backup_file"

    # Clear and restore
    docker run --rm \
        -v "${VOLUME_NAME}:/data" \
        -v "$(dirname "$backup_file"):/backup:ro" \
        busybox sh -c "rm -rf /data/* && tar xzf /backup/$(basename "$backup_file") -C /data"

    log_success "Restore complete"
    log_info "Start Neo4j with: pnpm infra:up"
}

do_list() {
    ensure_backup_dir

    echo ""
    echo "Available backups in $BACKUP_DIR:"
    echo ""

    if ls "$BACKUP_DIR"/neo4j-*.tar.gz &>/dev/null; then
        ls -lh "$BACKUP_DIR"/neo4j-*.tar.gz | awk '{print "  " $9 " (" $5 ")"}'
    else
        echo "  (no backups found)"
    fi
    echo ""
}

do_export_cypher() {
    log_info "Exporting data as Cypher..."

    local export_file="${BACKUP_DIR}/export-${TIMESTAMP}.cypher"
    ensure_backup_dir

    # Use neo4j container to run APOC export
    docker exec novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
        "CALL apoc.export.cypher.all('${export_file}', {format: 'cypher-shell'})" \
        2>/dev/null || {
            log_error "APOC export failed. Is APOC plugin installed?"
            log_info "Add to neo4j.conf: dbms.security.procedures.unrestricted=apoc.*"
            exit 1
        }

    log_success "Exported to: $export_file"
}

# Parse arguments
COMMAND="${1:-}"
shift || true

case "$COMMAND" in
    backup)
        name=""
        while [[ $# -gt 0 ]]; do
            case "$1" in
                --name) name="$2"; shift 2 ;;
                --volume) VOLUME_NAME="$2"; shift 2 ;;
                *) log_error "Unknown option: $1"; usage ;;
            esac
        done
        do_backup "$name"
        ;;
    restore)
        if [[ $# -lt 1 ]]; then
            log_error "Missing backup file"
            usage
        fi
        do_restore "$1"
        ;;
    list)
        do_list
        ;;
    export-cypher)
        do_export_cypher
        ;;
    *)
        usage
        ;;
esac
