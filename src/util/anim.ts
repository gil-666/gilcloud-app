import { Ref } from "vue";
const handleMouseMove = (e: MouseEvent, element: HTMLImageElement | null) => {
  if (!element) return;

  const rect = element?.getBoundingClientRect();
  const x = e.clientX - rect!.left; // cursor X within element
  const y = e.clientY - rect!.top;  // cursor Y within element

  // normalize to -1 .. 1
  const centerX = x / rect!.width - 0.5;
  const centerY = y / rect!.height - 0.5;

  const rotateX = centerY * -30; // tilt strength
  const rotateY = centerX * 30;

  element!.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
};

function resetTransform(element: HTMLImageElement | null) {
  if (element) {
    element!.style.transform = "perspective(1000px) rotateX(0) rotateY(0)";
  }
};
export { handleMouseMove, resetTransform };