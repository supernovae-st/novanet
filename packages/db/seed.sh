#!/bin/bash
# NovaNet - Neo4j Seed Script
# Exécute tous les fichiers .cypher dans l'ordre
#
# Usage (from monorepo root):
#   1. Start Neo4j: pnpm infra:up
#   2. Run seed:    pnpm infra:seed

set -euo pipefail

# UTF-8 encoding for proper diacritics support (é, ü, ñ, etc.)
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# SECURITY: Use environment variables with defaults for dev only
NEO4J_USER="${NEO4J_USER:-neo4j}"
NEO4J_PASSWORD="${NEO4J_PASSWORD:-novanetpassword}"
CONTAINER="${CONTAINER:-novanet-neo4j}"
SEED_DIR="$(dirname "$0")/seed"

# Couleurs
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${YELLOW}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}  NovaNet Neo4j Seed${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Vérifier que Neo4j est lancé
echo -e "${YELLOW}[1/3] Vérification de Neo4j...${NC}"
if ! docker ps --format '{{.Names}}' | grep -q "^${CONTAINER}$"; then
    echo -e "${RED}✗ Neo4j n'est pas lancé. Lance 'pnpm infra:up' depuis la racine du monorepo.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Neo4j est lancé${NC}"
echo ""

# Attendre que Neo4j soit prêt
echo -e "${YELLOW}[2/3] Attente que Neo4j soit prêt...${NC}"
MAX_ATTEMPTS=30
ATTEMPT=0
while ! docker exec "${CONTAINER}" cypher-shell -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" "RETURN 1" > /dev/null 2>&1; do
    ATTEMPT=$((ATTEMPT + 1))
    if [ $ATTEMPT -ge $MAX_ATTEMPTS ]; then
        echo -e "${RED}✗ Neo4j n'est pas prêt après ${MAX_ATTEMPTS} tentatives${NC}"
        exit 1
    fi
    echo "  Attente... ($ATTEMPT/$MAX_ATTEMPTS)"
    sleep 2
done
echo -e "${GREEN}✓ Neo4j est prêt${NC}"
echo ""

# Exécuter les fichiers .cypher dans l'ordre
echo -e "${YELLOW}[3/3] Exécution du seed...${NC}"
echo ""

for file in "$SEED_DIR"/*.cypher; do
    [ -f "$file" ] || continue
    filename=$(basename "$file")
    echo -e "  ${YELLOW}→ $filename${NC}"

    if docker exec -i "${CONTAINER}" cypher-shell -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" < "$file" > /dev/null 2>&1; then
        echo -e "    ${GREEN}✓ OK${NC}"
    else
        echo -e "    ${RED}✗ Erreur${NC}"
        echo ""
        echo "Détails de l'erreur:"
        docker exec -i "${CONTAINER}" cypher-shell -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" < "$file"
        exit 1
    fi
done

echo ""

# Exécuter les migrations (si présentes)
MIGRATION_DIR="$(dirname "$0")/migrations"
if [ -d "$MIGRATION_DIR" ]; then
    shopt -s nullglob
    migration_files=("$MIGRATION_DIR"/*.cypher)
    shopt -u nullglob
    if [ ${#migration_files[@]} -gt 0 ]; then
        echo -e "${YELLOW}[4/4] Exécution des migrations...${NC}"
        echo ""

        for file in "${migration_files[@]}"; do
            filename=$(basename "$file")
            echo -e "  ${YELLOW}→ $filename${NC}"

            if docker exec -i "${CONTAINER}" cypher-shell -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" < "$file" > /dev/null 2>&1; then
                echo -e "    ${GREEN}✓ OK${NC}"
            else
                echo -e "    ${RED}✗ Erreur${NC}"
                echo ""
                echo "Détails de l'erreur:"
                docker exec -i "${CONTAINER}" cypher-shell -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" < "$file"
                exit 1
            fi
        done
        echo ""
    fi
fi

echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  ✓ Seed terminé avec succès !${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Ouvrir Neo4j Browser: http://localhost:7474"
echo "Credentials: \$NEO4J_USER / \$NEO4J_PASSWORD (see environment)"
echo ""

# Stats rapides
echo -e "${YELLOW}Stats:${NC}"
docker exec "${CONTAINER}" cypher-shell -u "${NEO4J_USER}" -p "${NEO4J_PASSWORD}" \
    "MATCH (n) RETURN labels(n)[0] AS label, count(*) AS count ORDER BY count DESC" 2>/dev/null | tail -n +2
