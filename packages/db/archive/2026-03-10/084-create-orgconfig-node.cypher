// ============================================================================
// PLAN A - Migration 084: Create OrgConfig Node
// ============================================================================
// Priority: FOUNDATION (Must run first - other nodes depend on this)
// Fixes: Missing OrgConfig node in foundation layer
// CSR Impact: +1 node, enables org-level configuration
// ============================================================================

// Create the OrgConfig node for SuperNovae organization
MERGE (oc:OrgConfig {key: 'supernovae'})
ON CREATE SET
  oc.display_name = 'SuperNovae Studio',
  oc.description = 'Root organization configuration for SuperNovae AI ecosystem',
  oc.llm_context = 'USE: for organization-wide settings, branding, and project coordination. TRIGGERS: org, organization, company, supernovae, settings. NOT: project-specific config (use Project). RELATES: Project (child), Brand (owns).',
  oc.domain = 'supernovae.studio',
  oc.website_url = 'https://supernovae.studio',
  oc.industry = 'AI & Software Development',
  oc.created_at = datetime(),
  oc.updated_at = datetime()
ON MATCH SET
  oc.updated_at = datetime();

// Verify OrgConfig exists
MATCH (oc:OrgConfig {key: 'supernovae'})
RETURN oc.key AS key, oc.display_name AS name, 'OrgConfig created' AS status;
