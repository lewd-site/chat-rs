import { cubicOut } from 'svelte/easing';
import { TransitionConfig } from 'svelte/types/runtime/transition';

type EasingFunction = (t: number) => number;

interface SlideParams {
  delay: number;
  duration: number;
  easing: EasingFunction;
  origin: string;
}

export function hslide(node: Element, {
  delay = 0,
  duration = 400,
  easing = cubicOut,
  origin = '0 0'
}: SlideParams): TransitionConfig {
  const style = getComputedStyle(node);
  const opacity = +style.opacity;
  const height = parseFloat(style.height);

  return {
    delay,
    duration,
    easing,
    css: t =>
      `overflow: hidden;` +
      `opacity: ${Math.min(t * 20, 1) * opacity};` +
      `height: ${height}px;` +
      `transform: scaleX(${t});` +
      `transform-origin: ${origin};`
  };
}
