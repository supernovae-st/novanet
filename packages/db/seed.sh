#!/bin/bash
# NovaNet - Neo4j Seed Script
# Exécute tous les fichiers .cypher dans l'ordre
#
# Usage (from monorepo root):
#   1. Start Neo4j: npm run infra:up
#   2. Run seed:    cd core/neo4j && ./seed.sh

set -e

NEO4J_USER="neo4j"
NEO4J_PASSWORD="novanetpassword"
CONTAINER="novanet-neo4j"
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
    echo -e "${RED}✗ Neo4j n'est pas lancé. Lance 'npm run infra:up' depuis la racine du monorepo.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Neo4j est lancé${NC}"
echo ""

# Attendre que Neo4j soit prêt
echo -e "${YELLOW}[2/3] Attente que Neo4j soit prêt...${NC}"
MAX_ATTEMPTS=30
ATTEMPT=0
while ! docker exec $CONTAINER cypher-shell -u $NEO4J_USER -p $NEO4J_PASSWORD "RETURN 1" > /dev/null 2>&1; do
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

for file in $(ls -1 "$SEED_DIR"/*.cypher | sort); do
    filename=$(basename "$file")
    echo -e "  ${YELLOW}→ $filename${NC}"

    if cat "$file" | docker exec -i $CONTAINER cypher-shell -u $NEO4J_USER -p $NEO4J_PASSWORD > /dev/null 2>&1; then
        echo -e "    ${GREEN}✓ OK${NC}"
    else
        echo -e "    ${RED}✗ Erreur${NC}"
        echo ""
        echo "Détails de l'erreur:"
        cat "$file" | docker exec -i $CONTAINER cypher-shell -u $NEO4J_USER -p $NEO4J_PASSWORD
        exit 1
    fi
done

echo ""

# Exécuter les migrations (si présentes)
MIGRATION_DIR="$(dirname "$0")/migrations"
if [ -d "$MIGRATION_DIR" ] && [ "$(ls -A "$MIGRATION_DIR"/*.cypher 2>/dev/null)" ]; then
    echo -e "${YELLOW}[4/4] Exécution des migrations...${NC}"
    echo ""

    for file in $(ls -1 "$MIGRATION_DIR"/*.cypher | sort); do
        filename=$(basename "$file")
        echo -e "  ${YELLOW}→ $filename${NC}"

        if cat "$file" | docker exec -i $CONTAINER cypher-shell -u $NEO4J_USER -p $NEO4J_PASSWORD > /dev/null 2>&1; then
            echo -e "    ${GREEN}✓ OK${NC}"
        else
            echo -e "    ${RED}✗ Erreur${NC}"
            echo ""
            echo "Détails de l'erreur:"
            cat "$file" | docker exec -i $CONTAINER cypher-shell -u $NEO4J_USER -p $NEO4J_PASSWORD
            exit 1
        fi
    done
    echo ""
fi

echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  ✓ Seed terminé avec succès !${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Ouvrir Neo4j Browser: http://localhost:7474"
echo "Credentials: neo4j / novanetpassword"
echo ""

# Stats rapides
echo -e "${YELLOW}Stats:${NC}"
docker exec $CONTAINER cypher-shell -u $NEO4J_USER -p $NEO4J_PASSWORD \
    "MATCH (n) RETURN labels(n)[0] AS label, count(*) AS count ORDER BY count DESC" 2>/dev/null | tail -n +2
