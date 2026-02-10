import { NextResponse } from 'next/server';
import { readFile, writeFile } from 'fs/promises';
import { existsSync } from 'fs';
import path from 'path';
import os from 'os';

/**
 * API Route for Macropad Configuration
 *
 * Reads/writes config from: ~/Projects/work-louder/studio-integration/configs/work-louder-micro.json
 *
 * GET  /api/macropad/config - Read current config
 * POST /api/macropad/config - Save config to file
 */

// Resolve ~ to home directory
const CONFIG_PATH = path.join(
  os.homedir(),
  'Projects/work-louder/studio-integration/configs/work-louder-micro.json'
);

export async function GET() {
  try {
    // Check if file exists
    if (!existsSync(CONFIG_PATH)) {
      return NextResponse.json(
        { error: 'Config file not found', path: CONFIG_PATH },
        { status: 404 }
      );
    }

    // Read and parse the config file
    const content = await readFile(CONFIG_PATH, 'utf-8');
    const config = JSON.parse(content);

    return NextResponse.json(config);
  } catch (error) {
    console.error('[/api/macropad/config] GET error:', error);
    return NextResponse.json(
      { error: 'Failed to read config', details: String(error) },
      { status: 500 }
    );
  }
}

export async function POST(request: Request) {
  try {
    const config = await request.json();

    // Validate config structure
    if (!config.device || !config.layers) {
      return NextResponse.json(
        { error: 'Invalid config structure: missing device or layers' },
        { status: 400 }
      );
    }

    // Write config to file with pretty formatting
    await writeFile(CONFIG_PATH, JSON.stringify(config, null, 2), 'utf-8');

    return NextResponse.json({ success: true, path: CONFIG_PATH });
  } catch (error) {
    console.error('[/api/macropad/config] POST error:', error);
    return NextResponse.json(
      { error: 'Failed to save config', details: String(error) },
      { status: 500 }
    );
  }
}
