// 원 paint op의 원문·주소·전진을 바꾸지 않는 글리프 재생 뷰.
interface Candidate {start: number; end: number; glyph: string}
interface FontCoverage {
  getGlyphIDs(text: string): Uint16Array;
  getGlyphBounds?(glyphs: Uint16Array): Float32Array;
}
export function compatibilityGlyphView(text: readonly string[], positions: readonly number[],
  candidates: readonly Candidate[], font: FontCoverage): {text: string; points: string[]; positions: number[]} | null {
  if (positions.length !== text.length + 1 || candidates.length > text.length) return null;
  let previous = 0;
  for (const c of candidates) {
    const cp = c.glyph.codePointAt(0) ?? 0;
    if (!Number.isInteger(c.start) || !Number.isInteger(c.end) || c.start < previous
      || c.start >= c.end || c.end > text.length || [...c.glyph].length !== 1
      || cp < 0xe000 || cp > 0xf8ff) return null;
    previous = c.end;
  }
  const byStart = new Map(candidates.map(c => [c.start, c]));
  const points: string[] = [], replayPositions: number[] = [];
  for (let at = 0; at < text.length;) {
    const c = byStart.get(at), ids = c ? font.getGlyphIDs(c.glyph) : null;
    const bounds = ids && ids[0] ? font.getGlyphBounds?.(ids) : undefined;
    const visible = ids?.length === 1 && ids[0] !== 0
      && (!bounds || (bounds[2] > bounds[0] && bounds[3] > bounds[1]));
    replayPositions.push(positions[at]);
    if (c && visible) {points.push(c.glyph); at = c.end;}
    else {points.push(text[at]); at++;}
  }
  replayPositions.push(positions[text.length]);
  return {text: points.join(''), points, positions: replayPositions};
}
