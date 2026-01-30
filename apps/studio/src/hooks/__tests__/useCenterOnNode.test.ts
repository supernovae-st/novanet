// src/hooks/__tests__/useCenterOnNode.test.ts
import { calculateCenterOffset } from '../useCenterOnNode';
import { LAYOUT_CONSTANTS } from '@/config/layoutConstants';

describe('calculateCenterOffset', () => {
  const { BASE_MARGIN, TOP_BAR_HEIGHT, BOTTOM_BAR_HEIGHT, SIDEBAR_WIDTH, DETAILS_PANEL_WIDTH } = LAYOUT_CONSTANTS;

  it('should calculate offset to center node in visible area', () => {
    // With sidebar open (304px) and panel open (436px)
    // Horizontal: (304 - 436) / 2 = -66px (shift left because panel is wider)
    // Vertical: (136 - 96) / 2 = 20px (shift down because top is taller)
    const offset = calculateCenterOffset({
      sidebarOpen: true,
      focusMode: false,
      hasSelection: true,
    });

    const expectedLeft = SIDEBAR_WIDTH + BASE_MARGIN; // 304
    const expectedRight = DETAILS_PANEL_WIDTH + BASE_MARGIN; // 436
    const expectedTop = TOP_BAR_HEIGHT + BASE_MARGIN; // 136
    const expectedBottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN; // 96

    expect(offset.x).toBe((expectedLeft - expectedRight) / 2); // -66
    expect(offset.y).toBe((expectedTop - expectedBottom) / 2); // 20
  });

  it('should return zero offset in focus mode', () => {
    const offset = calculateCenterOffset({
      sidebarOpen: true,
      focusMode: true,
      hasSelection: true,
    });

    expect(offset.x).toBe(0);
    expect(offset.y).toBe(0);
  });

  it('should calculate offset with sidebar closed and panel open', () => {
    const offset = calculateCenterOffset({
      sidebarOpen: false,
      focusMode: false,
      hasSelection: true,
    });

    const expectedLeft = BASE_MARGIN; // 16
    const expectedRight = DETAILS_PANEL_WIDTH + BASE_MARGIN; // 436
    const expectedTop = TOP_BAR_HEIGHT + BASE_MARGIN; // 136
    const expectedBottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN; // 96

    expect(offset.x).toBe((expectedLeft - expectedRight) / 2); // -210
    expect(offset.y).toBe((expectedTop - expectedBottom) / 2); // 20
  });

  it('should calculate offset with sidebar open and no selection', () => {
    const offset = calculateCenterOffset({
      sidebarOpen: true,
      focusMode: false,
      hasSelection: false,
    });

    const expectedLeft = SIDEBAR_WIDTH + BASE_MARGIN; // 304
    const expectedRight = BASE_MARGIN; // 16
    const expectedTop = TOP_BAR_HEIGHT + BASE_MARGIN; // 136
    const expectedBottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN; // 96

    expect(offset.x).toBe((expectedLeft - expectedRight) / 2); // 144
    expect(offset.y).toBe((expectedTop - expectedBottom) / 2); // 20
  });

  it('should calculate symmetric offset with both panels closed', () => {
    const offset = calculateCenterOffset({
      sidebarOpen: false,
      focusMode: false,
      hasSelection: false,
    });

    const expectedLeft = BASE_MARGIN; // 16
    const expectedRight = BASE_MARGIN; // 16
    const expectedTop = TOP_BAR_HEIGHT + BASE_MARGIN; // 136
    const expectedBottom = BOTTOM_BAR_HEIGHT + BASE_MARGIN; // 96

    expect(offset.x).toBe((expectedLeft - expectedRight) / 2); // 0
    expect(offset.y).toBe((expectedTop - expectedBottom) / 2); // 20
  });
});
