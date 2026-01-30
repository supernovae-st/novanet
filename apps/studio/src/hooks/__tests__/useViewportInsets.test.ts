// src/hooks/__tests__/useViewportInsets.test.ts
import { calculateViewportInsets, LAYOUT_CONSTANTS } from '../useViewportInsets';

describe('calculateViewportInsets', () => {
  const { BASE_MARGIN, TOP_BAR_HEIGHT, BOTTOM_BAR_HEIGHT, SIDEBAR_WIDTH, DETAILS_PANEL_WIDTH } = LAYOUT_CONSTANTS;

  describe('Focus Mode', () => {
    it('should return minimal insets in focus mode', () => {
      const result = calculateViewportInsets({
        sidebarOpen: true,
        focusMode: true,
        hasSelection: true,
      });

      expect(result).toEqual({
        top: `${BASE_MARGIN}px`,
        right: `${BASE_MARGIN}px`,
        bottom: `${BASE_MARGIN}px`,
        left: `${BASE_MARGIN}px`,
      });
    });
  });

  describe('Normal Mode - Sidebar Open', () => {
    it('should account for sidebar when open, no selection', () => {
      const result = calculateViewportInsets({
        sidebarOpen: true,
        focusMode: false,
        hasSelection: false,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${SIDEBAR_WIDTH + BASE_MARGIN}px`,
      });
    });

    it('should account for sidebar and panel when both open', () => {
      const result = calculateViewportInsets({
        sidebarOpen: true,
        focusMode: false,
        hasSelection: true,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${DETAILS_PANEL_WIDTH + BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${SIDEBAR_WIDTH + BASE_MARGIN}px`,
      });
    });
  });

  describe('Normal Mode - Sidebar Closed', () => {
    it('should use minimal left inset when sidebar closed', () => {
      const result = calculateViewportInsets({
        sidebarOpen: false,
        focusMode: false,
        hasSelection: false,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${BASE_MARGIN}px`,
      });
    });

    it('should account for panel when sidebar closed but selection active', () => {
      const result = calculateViewportInsets({
        sidebarOpen: false,
        focusMode: false,
        hasSelection: true,
      });

      expect(result).toEqual({
        top: `${TOP_BAR_HEIGHT + BASE_MARGIN}px`,
        right: `${DETAILS_PANEL_WIDTH + BASE_MARGIN}px`,
        bottom: `${BOTTOM_BAR_HEIGHT + BASE_MARGIN}px`,
        left: `${BASE_MARGIN}px`,
      });
    });
  });
});
