declare module 'scrollama' {
  interface ScrollamaOptions {
    step: string;
    offset?: number;
    progress?: boolean;
    debug?: boolean;
    once?: boolean;
    threshold?: number;
    order?: boolean;
  }

  interface StepResponse {
    element: HTMLElement;
    index: number;
    direction: 'up' | 'down';
  }

  interface ProgressResponse extends StepResponse {
    progress: number;
  }

  interface ScrollamaInstance {
    setup(options: ScrollamaOptions): ScrollamaInstance;
    onStepEnter(callback: (response: StepResponse) => void): ScrollamaInstance;
    onStepExit(callback: (response: StepResponse) => void): ScrollamaInstance;
    onStepProgress(callback: (response: ProgressResponse) => void): ScrollamaInstance;
    resize(): void;
    enable(): void;
    disable(): void;
    destroy(): void;
    offsetTrigger(): number;
  }

  function scrollama(): ScrollamaInstance;
  export default scrollama;
}
