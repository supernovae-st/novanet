// novanet-core/src/parsers/parse-market.ts
import { LocaleMarket } from '../types/locale-knowledge.js';
import { parseMarkdownTable } from './utils.js';

/**
 * Get section content from header to next section
 * More robust section extraction that doesn't stop at table separators
 */
function getSection(content: string, header: string): string | null {
  const headerPattern = new RegExp(`### ${header.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}`, 'i');
  const headerMatch = content.match(headerPattern);
  if (!headerMatch || headerMatch.index === undefined) return null;

  const startIdx = headerMatch.index;
  const afterHeader = content.slice(startIdx);

  // Find the next section header (## or ###) or standalone divider (---\n)
  const nextSectionMatch = afterHeader.match(/\n(?=##[^#]|###\s|\n---\n)/);
  const endIdx = nextSectionMatch ? startIdx + (nextSectionMatch.index || afterHeader.length) : content.length;

  return content.slice(startIdx, endIdx);
}

/**
 * Parse age distribution from section 1.2
 */
function parseAgeDistribution(section: string): Array<{ group: string; percentage: number; notes: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    group: row.age_group || '',
    percentage: parseFloat(row.percentage?.replace('%', '') || '0'),
    notes: row.notes || '',
  })).filter(r => r.group);
}

/**
 * Parse income levels from section 1.3
 */
function parseIncomeLevels(section: string): Array<{ level: string; percentage: number; threshold: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    level: row.level || '',
    percentage: parseFloat(row.percentage?.replace('%', '') || '0'),
    threshold: row.monthly_income || '',
  })).filter(r => r.level);
}

/**
 * Parse urban/rural split from section 1.4
 */
function parseUrbanRuralSplit(section: string): Record<string, number> {
  const rows = parseMarkdownTable(section);
  const result: Record<string, number> = {};
  rows.forEach(row => {
    const type = row.type?.toLowerCase() || '';
    const percentage = parseFloat(row.percentage?.replace('%', '') || '0');
    if (type) {
      result[type] = percentage;
    }
  });
  return result;
}

/**
 * Parse payment methods from section 2.4
 */
function parsePaymentMethods(section: string): Array<{ method: string; usage: number; trend: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    method: row.method || '',
    usage: parseFloat(row.usage?.replace('%', '') || '0'),
    trend: row.trend || '',
  })).filter(r => r.method);
}

/**
 * Parse ROI factors from section 3.3
 */
function parseRoiFactors(section: string): Record<string, number> {
  const rows = parseMarkdownTable(section);
  const result: Record<string, number> = {};
  rows.forEach(row => {
    // Skip the overall row (which contains ** bold markers)
    const factor = row.factor?.toLowerCase().replace(/\s+/g, '_').replace(/\*/g, '') || '';
    const score = parseFloat(row['score_(0-100)'] || row.score || '0');
    if (factor && factor !== 'overall' && !factor.includes('**') && !isNaN(score)) {
      result[factor] = score;
    }
  });
  return result;
}

/**
 * Parse social platforms from section 4.1
 */
function parseSocialPlatforms(section: string): Array<{ platform: string; penetration: number; audience: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    platform: row.platform || '',
    penetration: parseFloat(row.penetration?.replace('%', '') || '0'),
    audience: row.primary_audience || row.best_for || '',
  })).filter(r => r.platform);
}

/**
 * Parse messaging apps from section 4.2
 */
function parseMessagingApps(section: string): Array<{ app: string; penetration: number; use_case: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    app: row.app || '',
    penetration: parseFloat(row.penetration?.replace('%', '') || '0'),
    use_case: row.use_case || '',
  })).filter(r => r.app);
}

/**
 * Parse search engines from section 4.4
 */
function parseSearchEngines(section: string): Array<{ engine: string; share: number }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    engine: row.engine || '',
    share: parseFloat(row.market_share?.replace('%', '') || '0'),
  })).filter(r => r.engine);
}

/**
 * Parse average order values from section 5.1
 */
function parseAvgOrderValue(section: string): Record<string, number> {
  const rows = parseMarkdownTable(section);
  const result: Record<string, number> = {};
  rows.forEach(row => {
    const category = row.category?.toLowerCase().replace(/\s+/g, '_') || '';
    const aov = parseFloat(row.aov?.replace(/[^\d.]/g, '') || '0');
    if (category) {
      result[category] = aov;
    }
  });
  return result;
}

/**
 * Parse dominant OS from section 2.2
 */
function parseDominantOs(section: string): Record<string, number> {
  const result: Record<string, number> = {};
  // Pattern: "Android 72% / iOS 28%"
  const osMatch = section.match(/Android\s+(\d+)%\s*\/?\s*iOS\s+(\d+)%/i);
  if (osMatch) {
    result['android'] = parseInt(osMatch[1], 10);
    result['ios'] = parseInt(osMatch[2], 10);
  }
  return result;
}

/**
 * Parse peak periods from section 6.1
 */
function parsePeakPeriods(section: string): Array<{ name: string; months: string; impact: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    name: row.period || '',
    months: row.months || '',
    impact: row.impact || '',
  })).filter(r => r.name);
}

/**
 * Parse low periods from section 6.2
 */
function parseLowPeriods(section: string): Array<{ name: string; strategy: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    name: row.period || '',
    strategy: row.strategy || '',
  })).filter(r => r.name);
}

/**
 * Parse shopping events from section 6.3
 */
function parseShoppingEvents(section: string): Array<{ event: string; date: string; impact: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    event: row.event || '',
    date: row.date || '',
    impact: row.revenue_impact || '',
  })).filter(r => r.event);
}

/**
 * Parse major players from section 7.1
 */
function parseMajorPlayers(section: string): Array<{ company: string; share: number; strength: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    company: row.company || '',
    share: parseFloat(row.market_share?.replace('%', '') || '0'),
    strength: row.strength || '',
  })).filter(r => r.company);
}

/**
 * Parse market concentration from section 7.2
 */
function parseMarketConcentration(section: string): 'fragmented' | 'moderate' | 'consolidated' {
  const typeMatch = section.match(/Market type\s*\|\s*(\w+)/i);
  if (typeMatch) {
    const type = typeMatch[1].toLowerCase();
    if (type.includes('fragment')) return 'fragmented';
    if (type.includes('consolidat')) return 'consolidated';
  }
  return 'moderate';
}

/**
 * Parse the market MD file and extract LocaleMarket data
 * @param content The raw markdown content
 * @returns Partial LocaleMarket object
 */
export function parseMarketMd(content: string): Partial<LocaleMarket> {
  const result: Partial<LocaleMarket> = {};

  // Section 1.1 - Population & Growth
  const populationMatch = content.match(/Total population\s*\|\s*([\d.,]+)\s*million/i);
  if (populationMatch) {
    result.population = parseFloat(populationMatch[1].replace(',', '.')) * 1000000;
  }

  const growthMatch = content.match(/Growth rate\s*\|\s*([\d.,]+)%/);
  if (growthMatch) {
    result.growth_rate = parseFloat(growthMatch[1].replace(',', '.'));
  }

  const medianAgeMatch = content.match(/Median age\s*\|\s*([\d.,]+)\s*years/i);
  if (medianAgeMatch) {
    result.median_age = parseFloat(medianAgeMatch[1].replace(',', '.'));
  }

  // Section 1.2 - Age Distribution
  const ageSection = getSection(content, '1.2 Age Distribution');
  if (ageSection) {
    result.age_distribution = parseAgeDistribution(ageSection);
  }

  // Section 1.3 - Income Levels
  const incomeSection = getSection(content, '1.3 Income Levels');
  if (incomeSection) {
    result.income_levels = parseIncomeLevels(incomeSection);
  }

  // Section 1.4 - Urban/Rural Split
  const urbanSection = getSection(content, '1.4 Urban/Rural Split');
  if (urbanSection) {
    result.urban_rural_split = parseUrbanRuralSplit(urbanSection);
  }

  // Section 2.1 - Internet Penetration
  const internetMatch = content.match(/Internet users\s*\|\s*(\d+)%/);
  if (internetMatch) {
    result.internet_penetration = parseInt(internetMatch[1], 10);
  }

  // Section 2.2 - Mobile Usage
  const mobileMatch = content.match(/Smartphone penetration\s*\|\s*(\d+)%/);
  if (mobileMatch) {
    result.mobile_penetration = parseInt(mobileMatch[1], 10);
  }

  const mobileFirstMatch = content.match(/Mobile-first users\s*\|\s*(\d+)%/);
  if (mobileFirstMatch) {
    result.mobile_first_users = parseInt(mobileFirstMatch[1], 10);
  }

  const mobileSection = getSection(content, '2.2 Mobile Usage');
  if (mobileSection) {
    result.dominant_os = parseDominantOs(mobileSection);
  }

  // Section 2.3 - E-commerce Adoption
  const ecommerceMatch = content.match(/Online shoppers\s*\|\s*(\d+)%/);
  if (ecommerceMatch) {
    result.ecommerce_adoption = parseInt(ecommerceMatch[1], 10);
  }

  const revenueMatch = content.match(/E-commerce revenue\s*\|\s*([\d.,]+)\s*billion/i);
  if (revenueMatch) {
    result.ecommerce_revenue = parseFloat(revenueMatch[1].replace(',', '.')) * 1000000000;
  }

  // Section 2.4 - Payment Methods
  const paymentSection = getSection(content, '2.4 Payment Methods');
  if (paymentSection) {
    result.payment_methods = parsePaymentMethods(paymentSection);
  }

  // Section 3.3 - ROI Priority Score
  const roiSection = getSection(content, '3.3 ROI Priority Score');
  if (roiSection) {
    result.roi_factors = parseRoiFactors(roiSection);

    // Extract overall ROI score
    const overallMatch = roiSection.match(/\*\*Overall\*\*\s*\|\s*\*\*([\d.]+)\*\*/);
    if (overallMatch) {
      result.roi_score = parseFloat(overallMatch[1]);
    }
  }

  // Section 4.1 - Social Media Platforms
  const socialSection = getSection(content, '4.1 Social Media Platforms');
  if (socialSection) {
    result.social_platforms = parseSocialPlatforms(socialSection);
  }

  // Section 4.2 - Messaging Apps
  const messagingSection = getSection(content, '4.2 Messaging Apps');
  if (messagingSection) {
    result.messaging_apps = parseMessagingApps(messagingSection);
  }

  // Section 4.4 - Search Engines
  const searchSection = getSection(content, '4.4 Search Engines');
  if (searchSection) {
    result.search_engines = parseSearchEngines(searchSection);
  }

  // Section 5.1 - Average Order Value
  const aovSection = getSection(content, '5.1 Average Order Value');
  if (aovSection) {
    result.avg_order_value = parseAvgOrderValue(aovSection);
  }

  // Section 5.2 - Conversion Patterns
  const conversionMatch = content.match(/Conversion rate\s*\|\s*([\d.]+)%/);
  if (conversionMatch) {
    result.conversion_rate = parseFloat(conversionMatch[1]);
  }

  const abandonmentMatch = content.match(/Cart abandonment\s*\|\s*(\d+)%/);
  if (abandonmentMatch) {
    result.cart_abandonment = parseInt(abandonmentMatch[1], 10);
  }

  // Section 6.1 - Peak Periods
  const peakSection = getSection(content, '6.1 Peak Periods');
  if (peakSection) {
    result.peak_periods = parsePeakPeriods(peakSection);
  }

  // Section 6.2 - Low Periods
  const lowSection = getSection(content, '6.2 Low Periods');
  if (lowSection) {
    result.low_periods = parseLowPeriods(lowSection);
  }

  // Section 6.3 - Key Shopping Events
  const eventsSection = getSection(content, '6.3 Key Shopping Events');
  if (eventsSection) {
    result.shopping_events = parseShoppingEvents(eventsSection);
  }

  // Section 7.1 - Major Players
  const playersSection = getSection(content, '7.1 Major Players');
  if (playersSection) {
    result.major_players = parseMajorPlayers(playersSection);
  }

  // Section 7.2 - Market Concentration
  const concentrationSection = getSection(content, '7.2 Market Concentration');
  if (concentrationSection) {
    result.market_concentration = parseMarketConcentration(concentrationSection);
  }

  return result;
}
