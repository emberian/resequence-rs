/**
 * Memorable entity names for the visualizer.
 * Uses sci-fi military codenames that are easy to distinguish.
 */

const prefixes = [
  'Alpha', 'Bravo', 'Charlie', 'Delta', 'Echo', 'Foxtrot', 'Golf', 'Hotel',
  'India', 'Juliet', 'Kilo', 'Lima', 'Mike', 'November', 'Oscar', 'Papa',
  'Quebec', 'Romeo', 'Sierra', 'Tango', 'Uniform', 'Victor', 'Whiskey',
  'Xray', 'Yankee', 'Zulu',
];

const suffixes = [
  'Prime', 'Nova', 'Vex', 'Flux', 'Core', 'Apex', 'Zero', 'One', 'Two',
  'Omega', 'Sigma', 'Tau', 'Phi', 'Rho', 'Psi', 'Chi', 'Eta', 'Zeta',
];

const colors = [
  'Crimson', 'Azure', 'Jade', 'Amber', 'Violet', 'Cobalt', 'Scarlet',
  'Onyx', 'Silver', 'Gold', 'Bronze', 'Ivory', 'Obsidian', 'Pearl',
];

/**
 * Generate a memorable entity name based on ID.
 * Uses NATO phonetic alphabet style with optional suffixes.
 */
export function getEntityName(id: number): string {
  const prefix = prefixes[id % prefixes.length];

  if (id < prefixes.length) {
    return prefix;
  }

  const suffixIndex = Math.floor(id / prefixes.length) - 1;
  const suffix = suffixes[suffixIndex % suffixes.length];

  return `${prefix} ${suffix}`;
}

/**
 * Get a short entity label (2-3 chars) based on ID.
 * Good for compact visualizations.
 */
export function getEntityLabel(id: number): string {
  const prefix = prefixes[id % prefixes.length];
  const letter = prefix[0];

  if (id < prefixes.length) {
    return letter;
  }

  const num = Math.floor(id / prefixes.length);
  return `${letter}${num}`;
}

/**
 * Get a color name for an entity based on its NameId.
 * Temporal duplicates (same NameId) get the same color.
 */
export function getEntityColor(nameId: number): string {
  return colors[nameId % colors.length];
}

/**
 * Generate a full descriptive name for chronoported entity.
 * E.g., "Alpha Prime (Crimson)" for entity 26 with nameId 0
 */
export function getFullEntityName(id: number, nameId: number): string {
  const name = getEntityName(id);
  const color = getEntityColor(nameId);
  return `${name} (${color})`;
}

/**
 * Wave names based on their speed characteristic.
 */
export function getWaveName(id: number, speed: number): string {
  if (id === 0) return 'Present';

  if (speed === 0) return 'Frozen';
  if (speed < 0) return 'Reverse';
  if (speed < 0.5) return 'Crawl';
  if (speed < 1) return 'Slow';
  if (speed === 1) return 'Normal';
  if (speed < 2) return 'Fast';
  return 'Hyper';
}

/**
 * Get wave display info including color based on speed.
 */
export function getWaveInfo(id: number, speed: number): { name: string; color: string } {
  const name = getWaveName(id, speed);

  let color: string;
  if (speed === 0) color = 'var(--text-muted)';
  else if (speed < 0) color = 'var(--color-paradox)';
  else if (speed < 1) color = 'var(--wave-slow)';
  else if (speed === 1) color = 'var(--wave-normal)';
  else color = 'var(--wave-fast)';

  return { name, color };
}
