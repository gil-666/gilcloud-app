const handleMouseMove = (e: MouseEvent, element: HTMLImageElement | null) => {
  if (!element) return;
  element.style.transition = "none";

  const rect = element.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const y = e.clientY - rect.top;

  const centerX = x / rect.width - 0.5;
  const centerY = y / rect.height - 0.5;

  const rotateX = centerY * -35;
  const rotateY = centerX * 35;

  // Read current scale from transform if present
  const currentTransform = element.style.transform;
  let scale = 1; // default
  const scaleMatch = currentTransform.match(/scale\(([^)]+)\)/);
  if (scaleMatch) scale = parseFloat(scaleMatch[1]);

  element.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale(${scale})`;
};

function albumArtOnPlay(element: HTMLImageElement | null) {
  if (!element) return;
  element.style.transition = "transform 0.3s ease";

  // Read current rotation and preserve it
  const currentTransform = element.style.transform;
  const rotateMatch = currentTransform.match(/rotateX\(([^)]+)\) rotateY\(([^)]+)\)/);
  const rotateX = rotateMatch ? rotateMatch[1] : "0deg";
  const rotateY = rotateMatch ? rotateMatch[2] : "0deg";

  element.style.transform = `perspective(1000px) ${rotateX ? `rotateX(${rotateX})` : ""} ${rotateY ? `rotateY(${rotateY})` : ""} scale(1.1)`;
}

function albumArtOnPause(element: HTMLImageElement | null) {
  if (!element) return;
  element.style.transition = "transform 0.3s ease";

  // Preserve rotation while scaling down
  const currentTransform = element.style.transform;
  const rotateMatch = currentTransform.match(/rotateX\(([^)]+)\) rotateY\(([^)]+)\)/);
  const rotateX = rotateMatch ? rotateMatch[1] : "0deg";
  const rotateY = rotateMatch ? rotateMatch[2] : "0deg";

  element.style.transform = `perspective(1000px) rotateX(${rotateX}) rotateY(${rotateY}) scale(0.9)`;
}

function resetTransform(element: HTMLImageElement | null) {
  if (element) {
    // Read current scale from transform if present
    const currentTransform = element.style.transform;
    let scale = 1; // default
    const scaleMatch = currentTransform.match(/scale\(([^)]+)\)/);
    if (scaleMatch) scale = parseFloat(scaleMatch[1]);
    element.style.transition = "transform 0.3s ease";
    element.style.transform = `perspective(1000px) rotateX(0deg) rotateY(0deg) scale(${scale})`;
  }
}

async function getPrimaryColor(img: HTMLImageElement): Promise<string> {
  if (!img.complete) {
    await new Promise<void>((resolve) => {
      img.onload = () => resolve();
      img.onerror = () => resolve(); // fallback
    });
  }

  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  if (!ctx) return "rgba(255,255,255,0.9)";

  const w = img.naturalWidth || img.width;
  const h = img.naturalHeight || img.height;
  canvas.width = w;
  canvas.height = h;

  ctx.drawImage(img, 0, 0, w, h);

  const imageData = ctx.getImageData(0, 0, w, h).data;
  const colorCount: Record<string, number> = {};
  let blackCount = 0;
  let totalCount = 0;

  for (let i = 0; i < imageData.length; i += 4 * 5) { // denser sampling
    const r = imageData[i];
    const g = imageData[i + 1];
    const b = imageData[i + 2];
    const a = imageData[i + 3];
    if (a < 128) continue; // ignore mostly transparent pixels
    totalCount++;
    if (r === 0 && g === 0 && b === 0) {
      blackCount++;
      continue; // ignore black for now
    }
    const key = `${r},${g},${b}`;
    colorCount[key] = (colorCount[key] || 0) + 1;
  }

  // Default to white if no colors counted
  if (Object.keys(colorCount).length === 0) {
    // if over 50% pixels are black, return black
    if (blackCount / totalCount > 0.5) return "rgba(0,0,0,0.9)";
    return "rgba(255,255,255,0.9)";
  }

  const primary = Object.entries(colorCount).reduce(
    (a, b) => (b[1] > a[1] ? b : a),
    ["255,255,255", 0]
  )[0];

  return `rgba(${primary},0.9)`;
}



async function applyGlow(img: HTMLImageElement | null) {
  if (!img) return;

  const color = await getPrimaryColor(img);
  const glow = `0px 0px 80px 17px ${color}`;
  img.style.transition = "box-shadow 0.3s ease";
  img.style.boxShadow = glow;
  img.style.webkitBoxShadow = glow;
  //@ts-ignore
  img.style.mozBoxShadow = glow;
}

export {
  albumArtOnPause, albumArtOnPlay, handleMouseMove, resetTransform, applyGlow
}