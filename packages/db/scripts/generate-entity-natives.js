#!/usr/bin/env node
/**
 * Generate EntityNatives for top 10 market locales
 *
 * Creates EntityNative nodes with proper denomination_forms per ADR-033
 * Generates: 26-entity-natives-extended.cypher
 */

const fs = require('fs');
const path = require('path');

const OUTPUT_FILE = path.join(__dirname, '..', 'seed', '26-entity-natives-extended.cypher');

// Top 10 market locales (en-US, fr-FR already exist)
const LOCALES_TO_ADD = ['es-ES', 'de-DE', 'pt-BR', 'ja-JP', 'zh-CN', 'ko-KR', 'it-IT', 'nl-NL'];

// Entity definitions with translations
const ENTITIES = {
  'qr-code': {
    'es-ES': { text: 'código qr', title: 'Código QR', abbrev: 'QR', url: 'codigo-qr', desc: 'Un código de barras bidimensional que almacena datos como un patrón de cuadrados, escaneable por smartphones.' },
    'de-DE': { text: 'qr-code', title: 'QR-Code', abbrev: 'QR', url: 'qr-code', desc: 'Ein zweidimensionaler Barcode, der Daten als Muster aus schwarzen und weißen Quadraten speichert.' },
    'pt-BR': { text: 'código qr', title: 'Código QR', abbrev: 'QR', url: 'codigo-qr', desc: 'Um código de barras bidimensional que armazena dados como padrão de quadrados, escaneável por smartphones.' },
    'ja-JP': { text: 'qrコード', title: 'QRコード', abbrev: 'QR', url: 'qr-code', desc: 'スマートフォンで読み取り可能な二次元バーコード。データを白黒のパターンで保存します。' },
    'zh-CN': { text: '二维码', title: '二维码', abbrev: 'QR', url: 'qr-code', desc: '一种二维条形码，以黑白方块图案存储数据，可被智能手机扫描。' },
    'ko-KR': { text: 'qr코드', title: 'QR코드', abbrev: 'QR', url: 'qr-code', desc: '스마트폰으로 스캔 가능한 2차원 바코드. 데이터를 흑백 패턴으로 저장합니다.' },
    'it-IT': { text: 'codice qr', title: 'Codice QR', abbrev: 'QR', url: 'codice-qr', desc: 'Un codice a barre bidimensionale che memorizza dati come pattern di quadrati, scansionabile da smartphone.' },
    'nl-NL': { text: 'qr-code', title: 'QR-code', abbrev: 'QR', url: 'qr-code', desc: 'Een tweedimensionale barcode die data opslaat als patroon van vierkanten, scanbaar met smartphones.' },
  },
  'dynamic-qr-code': {
    'es-ES': { text: 'código qr dinámico', title: 'Código QR Dinámico', abbrev: 'QR dinámico', url: 'codigo-qr-dinamico', desc: 'Un código QR editable que permite cambiar el destino sin reimprimir. Incluye seguimiento analítico.' },
    'de-DE': { text: 'dynamischer qr-code', title: 'Dynamischer QR-Code', abbrev: 'Dynamisch', url: 'dynamischer-qr-code', desc: 'Ein bearbeitbarer QR-Code, dessen Ziel ohne Neudruck geändert werden kann. Mit Analytics-Tracking.' },
    'pt-BR': { text: 'código qr dinâmico', title: 'Código QR Dinâmico', abbrev: 'QR dinâmico', url: 'codigo-qr-dinamico', desc: 'Um código QR editável que permite alterar o destino sem reimprimir. Inclui rastreamento analítico.' },
    'ja-JP': { text: 'ダイナミックqrコード', title: 'ダイナミックQRコード', abbrev: 'ダイナミック', url: 'dynamic-qr-code', desc: '再印刷なしで宛先を変更できる編集可能なQRコード。分析トラッキング付き。' },
    'zh-CN': { text: '动态二维码', title: '动态二维码', abbrev: '动态', url: 'dynamic-qr-code', desc: '可编辑的二维码，无需重新打印即可更改目标。包含分析跟踪功能。' },
    'ko-KR': { text: '다이내믹 qr코드', title: '다이내믹 QR코드', abbrev: '다이내믹', url: 'dynamic-qr-code', desc: '재인쇄 없이 목적지를 변경할 수 있는 편집 가능한 QR코드. 분석 추적 포함.' },
    'it-IT': { text: 'codice qr dinamico', title: 'Codice QR Dinamico', abbrev: 'Dinamico', url: 'codice-qr-dinamico', desc: 'Un codice QR modificabile che permette di cambiare destinazione senza ristampare. Include analytics.' },
    'nl-NL': { text: 'dynamische qr-code', title: 'Dynamische QR-code', abbrev: 'Dynamisch', url: 'dynamische-qr-code', desc: 'Een bewerkbare QR-code waarvan de bestemming kan worden gewijzigd zonder opnieuw af te drukken.' },
  },
  'static-qr-code': {
    'es-ES': { text: 'código qr estático', title: 'Código QR Estático', abbrev: 'QR estático', url: 'codigo-qr-estatico', desc: 'Un código QR fijo con datos codificados directamente. No se puede editar después de crearlo.' },
    'de-DE': { text: 'statischer qr-code', title: 'Statischer QR-Code', abbrev: 'Statisch', url: 'statischer-qr-code', desc: 'Ein fester QR-Code mit direkt codierten Daten. Nach der Erstellung nicht mehr bearbeitbar.' },
    'pt-BR': { text: 'código qr estático', title: 'Código QR Estático', abbrev: 'QR estático', url: 'codigo-qr-estatico', desc: 'Um código QR fixo com dados codificados diretamente. Não pode ser editado após a criação.' },
    'ja-JP': { text: '静的qrコード', title: '静的QRコード', abbrev: '静的', url: 'static-qr-code', desc: 'データが直接埋め込まれた固定QRコード。作成後は編集できません。' },
    'zh-CN': { text: '静态二维码', title: '静态二维码', abbrev: '静态', url: 'static-qr-code', desc: '数据直接编码的固定二维码。创建后无法编辑。' },
    'ko-KR': { text: '정적 qr코드', title: '정적 QR코드', abbrev: '정적', url: 'static-qr-code', desc: '데이터가 직접 인코딩된 고정 QR코드. 생성 후 편집할 수 없습니다.' },
    'it-IT': { text: 'codice qr statico', title: 'Codice QR Statico', abbrev: 'Statico', url: 'codice-qr-statico', desc: 'Un codice QR fisso con dati codificati direttamente. Non modificabile dopo la creazione.' },
    'nl-NL': { text: 'statische qr-code', title: 'Statische QR-code', abbrev: 'Statisch', url: 'statische-qr-code', desc: 'Een vaste QR-code met direct gecodeerde data. Kan niet worden bewerkt na aanmaak.' },
  },
  'custom-qr-code': {
    'es-ES': { text: 'código qr personalizado', title: 'Código QR Personalizado', abbrev: 'QR personalizado', url: 'codigo-qr-personalizado', desc: 'Un código QR con elementos visuales configurables: colores, logos, formas y marcos.' },
    'de-DE': { text: 'personalisierter qr-code', title: 'Personalisierter QR-Code', abbrev: 'Personalisiert', url: 'personalisierter-qr-code', desc: 'Ein QR-Code mit konfigurierbaren visuellen Elementen: Farben, Logos, Formen und Rahmen.' },
    'pt-BR': { text: 'código qr personalizado', title: 'Código QR Personalizado', abbrev: 'QR personalizado', url: 'codigo-qr-personalizado', desc: 'Um código QR com elementos visuais configuráveis: cores, logos, formas e molduras.' },
    'ja-JP': { text: 'カスタムqrコード', title: 'カスタムQRコード', abbrev: 'カスタム', url: 'custom-qr-code', desc: 'カスタマイズ可能な視覚要素を持つQRコード：色、ロゴ、形状、フレーム。' },
    'zh-CN': { text: '自定义二维码', title: '自定义二维码', abbrev: '自定义', url: 'custom-qr-code', desc: '具有可配置视觉元素的二维码：颜色、标志、形状和边框。' },
    'ko-KR': { text: '커스텀 qr코드', title: '커스텀 QR코드', abbrev: '커스텀', url: 'custom-qr-code', desc: '구성 가능한 시각 요소가 있는 QR코드: 색상, 로고, 모양, 프레임.' },
    'it-IT': { text: 'codice qr personalizzato', title: 'Codice QR Personalizzato', abbrev: 'Personalizzato', url: 'codice-qr-personalizzato', desc: 'Un codice QR con elementi visivi configurabili: colori, loghi, forme e cornici.' },
    'nl-NL': { text: 'aangepaste qr-code', title: 'Aangepaste QR-code', abbrev: 'Aangepast', url: 'aangepaste-qr-code', desc: "Een QR-code met configureerbare visuele elementen: kleuren, logo's, vormen en kaders." },
  },
  'qr-code-art': {
    'es-ES': { text: 'arte qr', title: 'Arte QR', abbrev: 'Arte QR', url: 'arte-qr', desc: 'Un código QR artístico generado por IA donde el patrón escaneable se fusiona con arte.' },
    'de-DE': { text: 'qr-code-kunst', title: 'QR-Code-Kunst', abbrev: 'QR-Kunst', url: 'qr-code-kunst', desc: 'Ein KI-generierter künstlerischer QR-Code, bei dem das scannbare Muster mit Kunst verschmilzt.' },
    'pt-BR': { text: 'arte qr', title: 'Arte QR', abbrev: 'Arte QR', url: 'arte-qr', desc: 'Um código QR artístico gerado por IA onde o padrão escaneável se funde com arte.' },
    'ja-JP': { text: 'qrコードアート', title: 'QRコードアート', abbrev: 'QRアート', url: 'qr-code-art', desc: 'スキャン可能なパターンがアートと融合したAI生成のアーティスティックQRコード。' },
    'zh-CN': { text: '二维码艺术', title: '二维码艺术', abbrev: 'QR艺术', url: 'qr-code-art', desc: 'AI生成的艺术二维码，可扫描图案与艺术作品融为一体。' },
    'ko-KR': { text: 'qr코드 아트', title: 'QR코드 아트', abbrev: 'QR아트', url: 'qr-code-art', desc: '스캔 가능한 패턴이 예술과 융합된 AI 생성 아티스틱 QR코드.' },
    'it-IT': { text: 'arte qr', title: 'Arte QR', abbrev: 'Arte QR', url: 'arte-qr', desc: "Un codice QR artistico generato da AI dove il pattern scansionabile si fonde con l'arte." },
    'nl-NL': { text: 'qr-code kunst', title: 'QR-code Kunst', abbrev: 'QR-kunst', url: 'qr-code-kunst', desc: 'Een door AI gegenereerde artistieke QR-code waarbij het scanbare patroon versmelt met kunst.' },
  },
  'qr-code-generator': {
    'es-ES': { text: 'generador de códigos qr', title: 'Generador de Códigos QR', abbrev: 'Generador QR', url: 'generador-codigos-qr', desc: 'Herramienta principal de creación de códigos QR con todos los formatos y opciones de exportación.' },
    'de-DE': { text: 'qr-code-generator', title: 'QR-Code-Generator', abbrev: 'QR-Generator', url: 'qr-code-generator', desc: 'Primäres QR-Code-Erstellungstool mit allen Formaten und Exportoptionen.' },
    'pt-BR': { text: 'gerador de código qr', title: 'Gerador de Código QR', abbrev: 'Gerador QR', url: 'gerador-codigo-qr', desc: 'Ferramenta principal de criação de códigos QR com todos os formatos e opções de exportação.' },
    'ja-JP': { text: 'qrコードジェネレーター', title: 'QRコードジェネレーター', abbrev: 'QR生成', url: 'qr-code-generator', desc: 'すべてのフォーマットとエクスポートオプションを備えた主要なQRコード作成ツール。' },
    'zh-CN': { text: '二维码生成器', title: '二维码生成器', abbrev: '生成器', url: 'qr-code-generator', desc: '支持所有格式和导出选项的主要二维码创建工具。' },
    'ko-KR': { text: 'qr코드 생성기', title: 'QR코드 생성기', abbrev: 'QR생성기', url: 'qr-code-generator', desc: '모든 형식과 내보내기 옵션을 지원하는 기본 QR코드 생성 도구.' },
    'it-IT': { text: 'generatore di codici qr', title: 'Generatore di Codici QR', abbrev: 'Generatore QR', url: 'generatore-codici-qr', desc: 'Strumento principale per la creazione di codici QR con tutti i formati e opzioni di esportazione.' },
    'nl-NL': { text: 'qr-code generator', title: 'QR-code Generator', abbrev: 'QR-generator', url: 'qr-code-generator', desc: 'Primaire QR-code creatietool met alle formaten en exportopties.' },
  },
  'barcode': {
    'es-ES': { text: 'código de barras', title: 'Código de Barras', abbrev: 'Barras', url: 'codigo-de-barras', desc: 'Formatos de códigos de barras lineales unidimensionales incluyendo EAN-13, UPC-A, Code 128.' },
    'de-DE': { text: 'barcode', title: 'Barcode', abbrev: 'Barcode', url: 'barcode', desc: 'Eindimensionale lineare Barcode-Formate einschließlich EAN-13, UPC-A, Code 128.' },
    'pt-BR': { text: 'código de barras', title: 'Código de Barras', abbrev: 'Barras', url: 'codigo-de-barras', desc: 'Formatos de códigos de barras lineares unidimensionais incluindo EAN-13, UPC-A, Code 128.' },
    'ja-JP': { text: 'バーコード', title: 'バーコード', abbrev: 'バーコード', url: 'barcode', desc: 'EAN-13、UPC-A、Code 128を含む一次元リニアバーコード形式。' },
    'zh-CN': { text: '条形码', title: '条形码', abbrev: '条码', url: 'barcode', desc: '一维线性条形码格式，包括EAN-13、UPC-A、Code 128。' },
    'ko-KR': { text: '바코드', title: '바코드', abbrev: '바코드', url: 'barcode', desc: 'EAN-13, UPC-A, Code 128을 포함한 1차원 선형 바코드 형식.' },
    'it-IT': { text: 'codice a barre', title: 'Codice a Barre', abbrev: 'Barre', url: 'codice-a-barre', desc: 'Formati di codici a barre lineari unidimensionali inclusi EAN-13, UPC-A, Code 128.' },
    'nl-NL': { text: 'barcode', title: 'Barcode', abbrev: 'Barcode', url: 'barcode', desc: 'Eendimensionale lineaire barcode-formaten inclusief EAN-13, UPC-A, Code 128.' },
  },
  'smart-link': {
    'es-ES': { text: 'enlace inteligente', title: 'Enlace Inteligente', abbrev: 'Smart Link', url: 'enlace-inteligente', desc: 'Una URL acortada inteligente con reglas de enrutamiento condicional y análisis.' },
    'de-DE': { text: 'smart link', title: 'Smart Link', abbrev: 'Smart Link', url: 'smart-link', desc: 'Eine intelligente verkürzte URL mit bedingten Routing-Regeln und Analysen.' },
    'pt-BR': { text: 'link inteligente', title: 'Link Inteligente', abbrev: 'Smart Link', url: 'link-inteligente', desc: 'Uma URL encurtada inteligente com regras de roteamento condicional e analytics.' },
    'ja-JP': { text: 'スマートリンク', title: 'スマートリンク', abbrev: 'スマートリンク', url: 'smart-link', desc: '条件付きルーティングルールと分析機能を備えたインテリジェントな短縮URL。' },
    'zh-CN': { text: '智能链接', title: '智能链接', abbrev: '智能链接', url: 'smart-link', desc: '具有条件路由规则和分析功能的智能短链接。' },
    'ko-KR': { text: '스마트 링크', title: '스마트 링크', abbrev: '스마트링크', url: 'smart-link', desc: '조건부 라우팅 규칙과 분석 기능이 있는 지능형 단축 URL.' },
    'it-IT': { text: 'link intelligente', title: 'Link Intelligente', abbrev: 'Smart Link', url: 'link-intelligente', desc: 'Un URL abbreviato intelligente con regole di routing condizionale e analytics.' },
    'nl-NL': { text: 'slimme link', title: 'Slimme Link', abbrev: 'Smart Link', url: 'slimme-link', desc: 'Een intelligente verkorte URL met voorwaardelijke routeringsregels en analytics.' },
  },
  'landing-page': {
    'es-ES': { text: 'página de destino', title: 'Página de Destino', abbrev: 'Landing', url: 'pagina-de-destino', desc: 'Una página de destino optimizada para móvil creada con constructor drag-and-drop.' },
    'de-DE': { text: 'landing page', title: 'Landing Page', abbrev: 'Landing', url: 'landing-page', desc: 'Eine für Mobilgeräte optimierte Zielseite mit Drag-and-Drop-Builder erstellt.' },
    'pt-BR': { text: 'página de destino', title: 'Página de Destino', abbrev: 'Landing', url: 'pagina-de-destino', desc: 'Uma página de destino otimizada para mobile criada com construtor drag-and-drop.' },
    'ja-JP': { text: 'ランディングページ', title: 'ランディングページ', abbrev: 'LP', url: 'landing-page', desc: 'ドラッグ＆ドロップビルダーで作成されたモバイル最適化ランディングページ。' },
    'zh-CN': { text: '落地页', title: '落地页', abbrev: '落地页', url: 'landing-page', desc: '通过拖放构建器创建的移动优化着陆页。' },
    'ko-KR': { text: '랜딩 페이지', title: '랜딩 페이지', abbrev: '랜딩', url: 'landing-page', desc: '드래그 앤 드롭 빌더로 만든 모바일 최적화 랜딩 페이지.' },
    'it-IT': { text: 'pagina di destinazione', title: 'Pagina di Destinazione', abbrev: 'Landing', url: 'pagina-di-destinazione', desc: 'Una pagina di destinazione ottimizzata per mobile creata con builder drag-and-drop.' },
    'nl-NL': { text: 'landingspagina', title: 'Landingspagina', abbrev: 'Landing', url: 'landingspagina', desc: 'Een voor mobiel geoptimaliseerde bestemmingspagina gemaakt met drag-and-drop builder.' },
  },
};

function escapeCypher(text) {
  if (!text) return '';
  return text
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/\n/g, ' ');
}

function generateDenominationForms(data) {
  return JSON.stringify([
    { type: 'text', value: data.text, priority: 1 },
    { type: 'title', value: data.title, priority: 1 },
    { type: 'abbrev', value: data.abbrev, priority: 1 },
    { type: 'url', value: data.url, priority: 1 },
  ]).replace(/'/g, "\\'");
}

async function main() {
  console.log('🔄 Generating EntityNatives for 8 additional locales...\n');

  const lines = [
    '// ============================================================================',
    '// Seed 26: EntityNatives Extended (8 locales × 9 entities = 72 nodes)',
    '// ============================================================================',
    '// Generated by: generate-entity-natives.js',
    `// Generated at: ${new Date().toISOString()}`,
    '// Locales: es-ES, de-DE, pt-BR, ja-JP, zh-CN, ko-KR, it-IT, nl-NL',
    '// ============================================================================',
    '',
  ];

  let count = 0;

  for (const locale of LOCALES_TO_ADD) {
    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push(`// ${locale} EntityNatives`);
    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push('');

    for (const [entitySlug, translations] of Object.entries(ENTITIES)) {
      const data = translations[locale];
      if (!data) continue;

      const key = `${entitySlug}@${locale}`;
      const entityKey = `entity:${entitySlug}`;
      const denomForms = generateDenominationForms(data);

      lines.push(`// ${data.title}`);
      lines.push(`MERGE (en:EntityNative {key: '${key}'})`);
      lines.push(`SET en.display_name = '${escapeCypher(data.title)}',`);
      lines.push(`    en.locale = '${locale}',`);
      lines.push(`    en.description = '${escapeCypher(data.desc)}',`);
      lines.push(`    en.denomination_forms = '${denomForms}',`);
      lines.push(`    en.provenance = 'authored',`);
      lines.push(`    en.created_at = datetime(),`);
      lines.push(`    en.updated_at = datetime();`);
      lines.push('');

      // Link to Entity
      lines.push(`MATCH (e:Entity {key: '${entityKey}'})`);
      lines.push(`MATCH (en:EntityNative {key: '${key}'})`);
      lines.push(`MERGE (e)-[:HAS_NATIVE]->(en);`);
      lines.push('');

      // Link to Locale
      lines.push(`MATCH (l:Locale {key: '${locale}'})`);
      lines.push(`MATCH (en:EntityNative {key: '${key}'})`);
      lines.push(`MERGE (en)-[:FOR_LOCALE]->(l);`);
      lines.push('');

      count++;
    }
  }

  fs.writeFileSync(OUTPUT_FILE, lines.join('\n'));

  console.log(`
═══════════════════════════════════════════════════════════════════
📊 ENTITYNATIVE GENERATION COMPLETE
═══════════════════════════════════════════════════════════════════
   Locales:         ${LOCALES_TO_ADD.length}
   Entities:        ${Object.keys(ENTITIES).length}
   EntityNatives:   ${count}

📁 Output: packages/db/seed/26-entity-natives-extended.cypher
`);
}

main().catch(console.error);
