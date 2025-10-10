#!/usr/bin/env node

import convert from 'color-convert';

/**
 * Inverts the lightness of an RGB color
 * @param {number} r - Red component (0-255)
 * @param {number} g - Green component (0-255)
 * @param {number} b - Blue component (0-255)
 * @returns {number[]} - Array of [invertedR, invertedG, invertedB]
 */
function invertRGB(r, g, b) {
  // Convert RGB to HSL
  const [h, s, l] = convert.rgb.hsl([r, g, b]);

  // Invert lightness
  const invertedL = 100 - l;

  // Convert back to RGB
  return convert.hsl.rgb([h, s, invertedL]);
}

// Get color from command line argument
const colorInput = process.argv[2];

if (!colorInput) {
  console.error('Usage: node color-invert.js <color>');
  console.error('Examples:');
  console.error('  node color-invert.js 007bff');
  console.error('  node color-invert.js #333');
  console.error('  node color-invert.js "rgba(0, 123, 255, 0.1)"');
  process.exit(1);
}

// Check if input is rgba() format
const rgbaMatch = colorInput.match(/rgba?\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*([\d.]+)\s*)?\)/);

if (rgbaMatch) {
  // Parse RGBA components
  const r = parseInt(rgbaMatch[1]);
  const g = parseInt(rgbaMatch[2]);
  const b = parseInt(rgbaMatch[3]);
  const alpha = rgbaMatch[4] ? parseFloat(rgbaMatch[4]) : 1;

  // Invert the RGB values
  const [invertedR, invertedG, invertedB] = invertRGB(r, g, b);

  // Output as rgba
  console.log(`rgba(${invertedR}, ${invertedG}, ${invertedB}, ${alpha})`);
} else {
  // Handle hex color format
  // Remove '#' if present
  let cleanHex = colorInput.replace('#', '');

  // Expand 3-digit hex to 6-digit hex (#333 -> #333333)
  if (/^[0-9A-Fa-f]{3}$/.test(cleanHex)) {
    cleanHex = cleanHex.split('').map(c => c + c).join('');
  }

  // Validate hex color
  if (!/^[0-9A-Fa-f]{6}$/.test(cleanHex)) {
    console.error('Invalid color format. Please provide:');
    console.error('  - 3 or 6-digit hex color (e.g., 333, 007bff, or #007bff)');
    console.error('  - rgba() format (e.g., "rgba(0, 123, 255, 0.1)")');
    process.exit(1);
  }

  // Split hex into RGB components and convert to decimal
  const r = parseInt(cleanHex.substring(0, 2), 16);
  const g = parseInt(cleanHex.substring(2, 4), 16);
  const b = parseInt(cleanHex.substring(4, 6), 16);

  // Invert the RGB values
  const [invertedR, invertedG, invertedB] = invertRGB(r, g, b);

  // Convert back to hex
  const invertedHex = [invertedR, invertedG, invertedB]
    .map(c => c.toString(16).padStart(2, '0'))
    .join('');

  console.log(`#${invertedHex}`);
}
