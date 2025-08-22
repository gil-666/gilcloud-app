import 'video.js';

declare module 'video.js' {
  interface Player {
    hlsQualitySelector(options?: any): any;
  }
}
