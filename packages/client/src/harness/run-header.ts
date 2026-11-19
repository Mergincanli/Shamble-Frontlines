/** What names a run, so that every reported number says where it was taken (PM-0002-1): machine, browser, display. */
export interface RunHeader {
  readonly commit: string;
  readonly label: string;
  readonly userAgent: string;
  readonly hardwareConcurrency: number;
  /** The display as the page sees it: CSS pixels, the device pixel ratio and the colour depth. */
  readonly display: string;
  readonly crossOriginIsolated: boolean;
  /** When the report was made, as an ISO time. */
  readonly at: string;
}

export function runHeader(target: Window, commit: string, label: string): RunHeader {
  const { screen, navigator } = target;
  return {
    commit,
    label,
    userAgent: navigator.userAgent,
    hardwareConcurrency: navigator.hardwareConcurrency,
    display: `${screen.width}x${screen.height} at ${target.devicePixelRatio}x, ${screen.colorDepth} bit`,
    crossOriginIsolated: target.crossOriginIsolated,
    at: new Date().toISOString(),
  };
}
