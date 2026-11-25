/**
 * The development readout: a corner box with a few lines of text and the "Copy report" button. Development builds
 * only; a production build never imports this file. Every text goes through `textContent` or a form field's value.
 */
export class Overlay {
  readonly #text: HTMLPreElement;
  readonly #box: HTMLDivElement;
  #fallback: HTMLTextAreaElement | null = null;

  constructor(parent: HTMLElement, makeReport: () => string) {
    const document = parent.ownerDocument;
    this.#box = document.createElement("div");
    this.#box.id = "overlay";
    this.#text = document.createElement("pre");
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = "Copy report";
    button.addEventListener("click", () => {
      // The button gives the focus back, so the game's keys go on working after a click on it.
      button.blur();
      const report = makeReport();
      // The clipboard can be refused (no permission, no focus): then the text is shown to be copied by hand.
      navigator.clipboard.writeText(report).then(
        () => {
          button.textContent = "Copied";
        },
        () => this.#showFallback(report),
      );
    });
    this.#box.append(this.#text, button);
    parent.append(this.#box);
  }

  /** Replaces the readout's lines. The caller decides how often: a few times a second, not every frame. */
  update(lines: readonly string[]): void {
    this.#text.textContent = lines.join("\n");
  }

  #showFallback(report: string): void {
    if (this.#fallback === null) {
      this.#fallback = this.#box.ownerDocument.createElement("textarea");
      this.#fallback.readOnly = true;
      this.#box.append(this.#fallback);
    }
    this.#fallback.value = report;
    this.#fallback.select();
  }
}
