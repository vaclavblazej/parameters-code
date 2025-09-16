// this is an altered version of svg-toolbelt https://github.com/zakariaf/svg-toolbelt
// distributed under the MIT License
var __defProp = Object.defineProperty;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => __defNormalProp(obj, typeof key !== "symbol" ? key + "" : key, value);
const DEFAULT_SVG_ENHANCER_CONFIG = Object.freeze(
  {
    minScale: 0.9,
    maxScale: 5,
    zoomStep: 0.3,
    transitionDuration: 200,
    showControls: true,
    controlsPosition: "top-right",
    enableTouch: true,
    enableKeyboard: true,
    showZoomLevelIndicator: true
  }
);
class EventEmitter {
  constructor() {
    __publicField(this, "events", /* @__PURE__ */ new Map());
  }
  on(event, listener) {
    if (!this.events.has(event)) {
      this.events.set(event, []);
    }
    this.events.get(event).push(listener);
    return this;
  }
  emit(event, ...args) {
    if (!this.events.has(event)) return false;
    for (const listener of this.events.get(event)) {
      try {
        listener(...args);
      } catch (err) {
        console.error(`Error in event listener for '${event}':`, err);
      }
    }
    return true;
  }
  off(event, listener) {
    if (!this.events.has(event)) return this;
    if (!listener) {
      this.events.delete(event);
      return this;
    }
    const listeners = this.events.get(event);
    const idx = listeners.indexOf(listener);
    if (idx > -1) {
      listeners.splice(idx, 1);
    }
    if (listeners.length === 0) {
      this.events.delete(event);
    }
    return this;
  }
  removeAllListeners() {
    this.events.clear();
    return this;
  }
}
const DEFAULT_FALLBACK_SVG_WIDTH = 400;
const DEFAULT_FALLBACK_SVG_HEIGHT = 300;
const CONTENT_SMALLER_THRESHOLD_FACTOR = 0.9;
const VISIBLE_CONTENT_FRACTION = 0.1;
class SvgEnhancer extends EventEmitter {
  constructor(container, config = {}) {
    super();
    __publicField(this, "container");
    __publicField(this, "svg");
    __publicField(this, "config");
    __publicField(this, "isDestroyed", false);
    __publicField(this, "scale", 1);
    __publicField(this, "translateX", 0);
    __publicField(this, "translateY", 0);
    __publicField(this, "features", {});
    __publicField(this, "transitionTimeoutId", null);
    this.container = container;
    this.config = { ...DEFAULT_SVG_ENHANCER_CONFIG, ...config };
    this.svg = container.querySelector("svg") || null;
    if (!this.svg) {
      this.isDestroyed = true;
      console.warn("SvgEnhancer: No <svg> found in container");
      return;
    }
  }
  /**
   * Initialize the container (e.g., add CSS classes), and then features will hook in.
   */
  init() {
    if (this.isDestroyed) return;
    this.setupContainer();
  }
  setupContainer() {
    this.container.classList.add("svg-toolbelt-container");
    this.svg.classList.add("svg-toolbelt-svg");
  }
  /**
   * Ensure panning does not exceed reasonable limits based on content and zoom level.
   * Allows panning until only 10% of content remains visible.
   */
  constrainPan() {
    if (!this.svg) return;
    const containerRect = this.containerRect;
    const svgBounds = this._getSvgBounds();
    const scaledWidth = svgBounds.width * this.scale;
    const scaledHeight = svgBounds.height * this.scale;
    let maxTranslateX;
    if (scaledWidth < containerRect.width * CONTENT_SMALLER_THRESHOLD_FACTOR) {
      maxTranslateX = containerRect.width;
    } else {
      const minVisibleWidth = scaledWidth * VISIBLE_CONTENT_FRACTION;
      maxTranslateX = scaledWidth - minVisibleWidth;
    }
    let maxTranslateY;
    if (scaledHeight < containerRect.height * CONTENT_SMALLER_THRESHOLD_FACTOR) {
      maxTranslateY = containerRect.height;
    } else {
      const minVisibleHeight = scaledHeight * VISIBLE_CONTENT_FRACTION;
      maxTranslateY = scaledHeight - minVisibleHeight;
    }
    this.translateX = Math.max(-maxTranslateX, Math.min(maxTranslateX, this.translateX));
    this.translateY = Math.max(-maxTranslateY, Math.min(maxTranslateY, this.translateY));
  }
  /**
   * Destroy all features and remove listeners.
   */
  destroy() {
    if (this.isDestroyed) return;
    this.isDestroyed = true;
    if (this.transitionTimeoutId !== null) {
      clearTimeout(this.transitionTimeoutId);
      this.transitionTimeoutId = null;
    }
    Object.values(this.features).forEach((feature) => {
      if (typeof (feature == null ? void 0 : feature.destroy) === "function") {
        feature.destroy();
      }
    });
    this.removeAllListeners();
    this.features = {};
  }
  /**
   * Convenience getter for the container's bounding rect.
   */
  get containerRect() {
    return this.container.getBoundingClientRect();
  }
  /**
   * Reset the SVG to its default scale and position with transition animation.
   */
  reset() {
    if (this.isDestroyed || !this.svg) return;
    this.scale = 1;
    this.translateX = 0;
    this.translateY = 0;
    this.applyTransformWithTransition();
    this.emit("reset", {
      translateX: this.translateX,
      translateY: this.translateY,
      scale: this.scale
    });
  }
  /**
   * Apply the current transform to the SVG without transition animation.
   */
  applyTransform() {
    if (this.isDestroyed || !this.svg) return;
    this.svg.style.transition = "none";
    this.svg.style.transform = `translate(${this.translateX}px, ${this.translateY}px) scale(${this.scale})`;
  }
  /**
   * Apply the current transform to the SVG with transition animation.
   */
  applyTransformWithTransition() {
    if (this.isDestroyed || !this.svg) return;
    if (this.transitionTimeoutId !== null) {
      clearTimeout(this.transitionTimeoutId);
    }
    this.svg.style.transition = `transform ${this.config.transitionDuration}ms ease-out`;
    this.svg.style.transform = `translate(${this.translateX}px, ${this.translateY}px) scale(${this.scale})`;
    this.transitionTimeoutId = window.setTimeout(() => {
      if (!this.isDestroyed && this.svg) {
        this.svg.style.transition = "none";
      }
      this.transitionTimeoutId = null;
    }, this.config.transitionDuration);
  }
  /**
   * Try to get SVG bounds using getBBox method (works in real browsers)
   */
  _tryGetBoundsFromBBox() {
    try {
      if (typeof this.svg.getBBox === "function") {
        const bbox = this.svg.getBBox();
        return { width: bbox.width, height: bbox.height };
      }
    } catch {
    }
    return null;
  }
  /**
   * Try to get SVG bounds from viewBox attribute
   */
  _tryGetBoundsFromViewBox() {
    var _a;
    try {
      const viewBox = (_a = this.svg.viewBox) == null ? void 0 : _a.baseVal;
      if (viewBox && typeof viewBox.width === "number" && typeof viewBox.height === "number" && !isNaN(viewBox.width) && !isNaN(viewBox.height) && viewBox.width > 0 && viewBox.height > 0) {
        return { width: viewBox.width, height: viewBox.height };
      }
    } catch {
    }
    return null;
  }
  /**
   * Get SVG bounds from width/height attributes or fallback to defaults
   */
  _getBoundsFromAttributesOrDefault() {
    const svgWidthAttr = this.svg.getAttribute("width");
    const svgHeightAttr = this.svg.getAttribute("height");
    let w = NaN;
    let h = NaN;
    if (svgWidthAttr) {
      const parsedWidth = parseFloat(svgWidthAttr);
      w = !isNaN(parsedWidth) && parsedWidth > 0 ? parsedWidth : NaN;
    }
    if (svgHeightAttr) {
      const parsedHeight = parseFloat(svgHeightAttr);
      h = !isNaN(parsedHeight) && parsedHeight > 0 ? parsedHeight : NaN;
    }
    return {
      width: isNaN(w) ? DEFAULT_FALLBACK_SVG_WIDTH : w,
      height: isNaN(h) ? DEFAULT_FALLBACK_SVG_HEIGHT : h
    };
  }
  /**
   * Get SVG bounds using fallback chain: getBBox -> viewBox -> attributes -> defaults
   */
  _getSvgBounds() {
    let bounds = this._tryGetBoundsFromBBox();
    if (bounds) return bounds;
    bounds = this._tryGetBoundsFromViewBox();
    if (bounds) return bounds;
    return this._getBoundsFromAttributesOrDefault();
  }
}
class ZoomFeature {
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "handleWheel");
    this.enhancer = enhancer;
    this.handleWheel = this._handleWheel.bind(this);
  }
  init() {
    this.enhancer.container.addEventListener("wheel", this.handleWheel, {
      passive: false
    });
  }
  _handleWheel(e) {
    if (this.enhancer.isDestroyed) return;
    e.preventDefault();
    const rect = this.enhancer.containerRect;
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;
    const delta = e.deltaY > 0 ? -this.enhancer.config.zoomStep : this.enhancer.config.zoomStep;
    this.zoomAt(mouseX, mouseY, delta);
  }
  zoomIn() {
    const rect = this.enhancer.containerRect;
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;
    this.zoomAt(centerX, centerY, this.enhancer.config.zoomStep);
  }
  zoomOut() {
    const rect = this.enhancer.containerRect;
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;
    this.zoomAt(centerX, centerY, -this.enhancer.config.zoomStep);
  }
  zoomAt(x, y, delta) {
    if (this.enhancer.isDestroyed) return;
    const newScale = Math.max(
      this.enhancer.config.minScale,
      Math.min(this.enhancer.config.maxScale, this.enhancer.scale + delta)
    );
    if (newScale === this.enhancer.scale) return;
    const svgX = (x - this.enhancer.translateX) / this.enhancer.scale;
    const svgY = (y - this.enhancer.translateY) / this.enhancer.scale;
    this.enhancer.scale = newScale;
    this.enhancer.translateX = x - svgX * this.enhancer.scale;
    this.enhancer.translateY = y - svgY * this.enhancer.scale;
    this.enhancer.constrainPan();
    this.enhancer.emit("zoom", {
      translateX: this.enhancer.translateX,
      translateY: this.enhancer.translateY,
      scale: this.enhancer.scale
    });
    this.enhancer.svg.style.transform = `translate(${this.enhancer.translateX}px, ${this.enhancer.translateY}px) scale(${this.enhancer.scale})`;
  }
  destroy() {
    this.enhancer.container.removeEventListener("wheel", this.handleWheel);
  }
}
class PanFeature {
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "isDragging", false);
    __publicField(this, "lastMouseX", 0);
    __publicField(this, "lastMouseY", 0);
    __publicField(this, "handleMouseDown");
    __publicField(this, "handleMouseMove");
    __publicField(this, "handleMouseUp");
    this.enhancer = enhancer;
    this.handleMouseDown = this._handleMouseDown.bind(this);
    this.handleMouseMove = this._handleMouseMove.bind(this);
    this.handleMouseUp = this._handleMouseUp.bind(this);
  }
  init() {
    this.enhancer.svg.addEventListener("mousedown", this.handleMouseDown);
    document.addEventListener("mousemove", this.handleMouseMove);
    document.addEventListener("mouseup", this.handleMouseUp);
  }
  _handleMouseDown(e) {
    if (this.enhancer.isDestroyed || e.button !== 0) return;
    e.preventDefault();
    this.isDragging = true;
    this.lastMouseX = e.clientX;
    this.lastMouseY = e.clientY;
  }
  _handleMouseMove(e) {
    if (this.enhancer.isDestroyed || !this.isDragging) return;
    e.preventDefault();
    const deltaX = e.clientX - this.lastMouseX;
    const deltaY = e.clientY - this.lastMouseY;
    this.enhancer.translateX += deltaX;
    this.enhancer.translateY += deltaY;
    this.lastMouseX = e.clientX;
    this.lastMouseY = e.clientY;
    this.enhancer.constrainPan();
    this.enhancer.emit("pan", {
      translateX: this.enhancer.translateX,
      translateY: this.enhancer.translateY,
      scale: this.enhancer.scale
    });
    this.enhancer.svg.style.transform = `translate(${this.enhancer.translateX}px, ${this.enhancer.translateY}px) scale(${this.enhancer.scale})`;
  }
  _handleMouseUp(_event) {
    if (this.enhancer.isDestroyed || !this.isDragging) return;
    this.isDragging = false;
  }
  destroy() {
    this.enhancer.svg.removeEventListener("mousedown", this.handleMouseDown);
    document.removeEventListener("mousemove", this.handleMouseMove);
    document.removeEventListener("mouseup", this.handleMouseUp);
  }
}
function getTouchDistance(touch1, touch2) {
  const dx = touch1.clientX - touch2.clientX;
  const dy = touch1.clientY - touch2.clientY;
  return Math.sqrt(dx * dx + dy * dy);
}
function getTouchCenter(touch1, touch2) {
  return {
    x: (touch1.clientX + touch2.clientX) / 2,
    y: (touch1.clientY + touch2.clientY) / 2
  };
}
function createControlButton(text, title, onClick) {
  const button = document.createElement("button");
  button.textContent = text;
  button.title = title;
  button.className = "svg-toolbelt-btn";
  button.addEventListener("click", (e) => {
    e.preventDefault();
    e.stopPropagation();
    onClick();
  });
  return button;
}
class TouchFeature {
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "isDragging", false);
    __publicField(this, "lastMouseX", 0);
    __publicField(this, "lastMouseY", 0);
    __publicField(this, "lastTouchDistance", 0);
    __publicField(this, "handleTouchStart");
    __publicField(this, "handleTouchMove");
    __publicField(this, "handleTouchEnd");
    this.enhancer = enhancer;
    this.handleTouchStart = this._handleTouchStart.bind(this);
    this.handleTouchMove = this._handleTouchMove.bind(this);
    this.handleTouchEnd = this._handleTouchEnd.bind(this);
  }
  init() {
    const svgEl = this.enhancer.svg;
    svgEl.addEventListener("touchstart", this.handleTouchStart, {
      passive: false
    });
    svgEl.addEventListener("touchmove", this.handleTouchMove, {
      passive: false
    });
    svgEl.addEventListener("touchend", this.handleTouchEnd);
  }
  _handleTouchStart(e) {
    if (this.enhancer.isDestroyed) return;
    e.preventDefault();
    if (!e.touches || typeof e.touches.length !== "number") {
      return;
    }
    if (e.touches.length === 1) {
      this.isDragging = true;
      this.lastMouseX = e.touches[0].clientX;
      this.lastMouseY = e.touches[0].clientY;
    } else if (e.touches.length === 2) {
      this.isDragging = false;
      const [t1, t2] = [e.touches[0], e.touches[1]];
      this.lastTouchDistance = getTouchDistance(t1, t2);
    }
  }
  _handleTouchMove(e) {
    if (this.enhancer.isDestroyed) return;
    e.preventDefault();
    if (!e.touches || typeof e.touches.length !== "number") {
      return;
    }
    if (e.touches.length === 1 && this.isDragging) {
      const touch = e.touches[0];
      const deltaX = touch.clientX - this.lastMouseX;
      const deltaY = touch.clientY - this.lastMouseY;
      this.enhancer.translateX += deltaX;
      this.enhancer.translateY += deltaY;
      this.lastMouseX = touch.clientX;
      this.lastMouseY = touch.clientY;
      this.enhancer.constrainPan();
      this.enhancer.svg.style.transform = `translate(${this.enhancer.translateX}px, ${this.enhancer.translateY}px) scale(${this.enhancer.scale})`;
    } else if (e.touches.length === 2) {
      const [t1, t2] = [e.touches[0], e.touches[1]];
      const distance = getTouchDistance(t1, t2);
      const center = getTouchCenter(t1, t2);
      if (this.lastTouchDistance > 0) {
        const scaleDelta = (distance / this.lastTouchDistance - 1) * this.enhancer.scale;
        const rect = this.enhancer.containerRect;
        const centerX = center.x - rect.left;
        const centerY = center.y - rect.top;
        this.enhancer.features.zoom.zoomAt(centerX, centerY, scaleDelta);
      }
      this.lastTouchDistance = distance;
    }
  }
  _handleTouchEnd(_event) {
    this.isDragging = false;
  }
  destroy() {
    const svgEl = this.enhancer.svg;
    svgEl.removeEventListener("touchstart", this.handleTouchStart);
    svgEl.removeEventListener("touchmove", this.handleTouchMove);
    svgEl.removeEventListener("touchend", this.handleTouchEnd);
  }
}
class KeyboardFeature {
  // Add isDestroyed property
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "handleKeyDown");
    __publicField(this, "isDestroyed", false);
    this.enhancer = enhancer;
    this.handleKeyDown = this._handleKeyDown.bind(this);
  }
  init() {
    this.enhancer.container.setAttribute("tabindex", "0");
    this.enhancer.container.addEventListener("keydown", this.handleKeyDown);
  }
  _handleKeyDown(e) {
    if (this.enhancer.isDestroyed) return;
    const step = 20;
    switch (true) {
      case (e.key === "+" || e.key === "="):
        e.preventDefault();
        this.enhancer.features.zoom.zoomIn();
        break;
      case e.key === "-":
        e.preventDefault();
        this.enhancer.features.zoom.zoomOut();
        break;
      case e.key === "0":
        e.preventDefault();
        this.enhancer.reset();
        break;
      case e.key === "ArrowUp":
        e.preventDefault();
        this.enhancer.translateY += step;
        this.enhancer.constrainPan();
        this.emitArrowEvent();
        this.enhancer.applyTransform();
        break;
      case e.key === "ArrowDown":
        e.preventDefault();
        this.enhancer.translateY -= step;
        this.enhancer.constrainPan();
        this.emitArrowEvent();
        this.enhancer.applyTransform();
        break;
      case e.key === "ArrowLeft":
        e.preventDefault();
        this.enhancer.translateX += step;
        this.enhancer.constrainPan();
        this.emitArrowEvent();
        this.enhancer.applyTransform();
        break;
      case e.key === "ArrowRight":
        e.preventDefault();
        this.enhancer.translateX -= step;
        this.enhancer.constrainPan();
        this.emitArrowEvent();
        this.enhancer.applyTransform();
        break;
    }
  }
  // Helper function to emit arrow event
  emitArrowEvent() {
    this.enhancer.emit("arrow", {
      translateX: this.enhancer.translateX,
      translateY: this.enhancer.translateY,
      scale: this.enhancer.scale
    });
  }
  destroy() {
    this.enhancer.container.removeEventListener("keydown", this.handleKeyDown);
    this.isDestroyed = true;
  }
}
class ControlsFeature {
  // Add isDestroyed property
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "controlsContainer", null);
    __publicField(this, "isDestroyed", false);
    this.enhancer = enhancer;
  }
  init() {
    this.createControls();
  }
  createControls() {
    const container = document.createElement("div");
    container.className = `svg-toolbelt-controls position-${this.enhancer.config.controlsPosition}`;
    const zoomInBtn = createControlButton(
      "+",
      "Zoom In",
      () => this.enhancer.features.zoom.zoomIn()
    );
    const zoomOutBtn = createControlButton(
      "−",
      "Zoom Out",
      () => this.enhancer.features.zoom.zoomOut()
    );
    const resetBtn = createControlButton("⌂", "Reset Zoom", () => {
      this.enhancer.reset();
    });
    if (document.fullscreenEnabled && this.enhancer.features.fullscreen) {
      const fsBtn = createControlButton(
        "⛶",
        "Toggle Fullscreen",
        () => this.enhancer.features.fullscreen.toggleFullscreen()
      );
      container.appendChild(fsBtn);
    }
    container.appendChild(zoomInBtn);
    container.appendChild(zoomOutBtn);
    container.appendChild(resetBtn);
    this.enhancer.container.appendChild(container);
    this.controlsContainer = container;
  }
  destroy() {
    if (this.controlsContainer) {
      this.controlsContainer.remove();
      this.controlsContainer = null;
    }
    this.isDestroyed = true;
  }
}
class FullscreenFeature {
  constructor(enhancer) {
    __publicField(this, "enhancer");
    this.enhancer = enhancer;
  }
  toggleFullscreen() {
    if (this.enhancer.isDestroyed) return;
    if (document.fullscreenElement) {
      if (typeof document.exitFullscreen === "function") {
        document.exitFullscreen().catch(() => {
          console.warn("Failed to exit fullscreen");
        });
      } else {
        console.warn("exitFullscreen() is not supported in this environment");
      }
    } else {
      const containerEl = this.enhancer.container;
      if (typeof containerEl.requestFullscreen === "function") {
        containerEl.requestFullscreen().catch((err) => {
          console.warn("Failed to enter fullscreen:", err);
        });
      } else {
        console.warn(
          "requestFullscreen() is not supported in this environment"
        );
      }
    }
  }
  destroy() {
  }
}
class DblclickResetFeature {
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "handleDblClick");
    this.enhancer = enhancer;
    this.handleDblClick = this._handleDblClick.bind(this);
  }
  init() {
    this.enhancer.svg.addEventListener("dblclick", this.handleDblClick);
  }
  _handleDblClick(_event) {
    if (!this.enhancer.isDestroyed) {
      this.enhancer.reset();
    }
  }
  destroy() {
    this.enhancer.svg.removeEventListener("dblclick", this.handleDblClick);
  }
}
class NoContextMenuFeature {
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "handleContextMenu");
    this.enhancer = enhancer;
    this.handleContextMenu = (e) => e.preventDefault();
  }
  init() {
    this.enhancer.container.addEventListener(
      "contextmenu",
      this.handleContextMenu
    );
  }
  destroy() {
    this.enhancer.container.removeEventListener(
      "contextmenu",
      this.handleContextMenu
    );
  }
}
class ZoomLevelIndicatorFeature {
  constructor(enhancer) {
    __publicField(this, "enhancer");
    __publicField(this, "badge");
    __publicField(this, "hideTimeout", null);
    __publicField(this, "onZoom");
    this.enhancer = enhancer;
    this.onZoom = this._onZoom.bind(this);
    this.badge = document.createElement("div");
    this.badge.className = "svg-toolbelt-zoom-indicator";
    this.badge.style.opacity = "0";
    this.badge.setAttribute("aria-live", "polite");
    this.badge.setAttribute("aria-label", "Current zoom level");
  }
  init() {
    this.enhancer.container.appendChild(this.badge);
    this.enhancer.on("zoom", this.onZoom);
  }
  _onZoom(event) {
    if (this.enhancer.isDestroyed) return;
    const percent = `${Math.round(event.scale * 100)}%`;
    this.badge.textContent = percent;
    this.badge.style.opacity = "1";
    if (this.hideTimeout) {
      window.clearTimeout(this.hideTimeout);
    }
    this.hideTimeout = window.setTimeout(() => {
      if (!this.enhancer.isDestroyed) {
        this.badge.style.opacity = "0";
      }
    }, 1500);
  }
  destroy() {
    this.enhancer.off("zoom", this.onZoom);
    if (this.hideTimeout) {
      window.clearTimeout(this.hideTimeout);
      this.hideTimeout = null;
    }
    if (this.badge.parentNode) {
      this.badge.parentNode.removeChild(this.badge);
    }
  }
}
class SvgToolbelt extends SvgEnhancer {
  constructor(container, config) {
    super(container, config);
    if (this.isDestroyed) return;
    this.features = {
      zoom: new ZoomFeature(this),
      pan: new PanFeature(this),
      dblclickReset: new DblclickResetFeature(this),
      noContextMenu: new NoContextMenuFeature(this),
      touch: this.config.enableTouch ? new TouchFeature(this) : null,
      keyboard: this.config.enableKeyboard ? new KeyboardFeature(this) : null,
      controls: this.config.showControls ? new ControlsFeature(this) : null,
      fullscreen: document.fullscreenEnabled ? new FullscreenFeature(this) : null,
      zoomLevelIndicator: this.config.showZoomLevelIndicator ? new ZoomLevelIndicatorFeature(this) : null
    };
  }
  /**
   * Initialize all features, then apply any initial transforms.
   */
  init() {
    if (this.isDestroyed || !this.svg) return;
    super.init();
    Object.values(this.features).forEach((feature) => {
      if (feature && typeof feature.init === "function") {
        feature.init();
      }
    });
    this.svg.style.transform = `translate(${this.translateX}px, ${this.translateY}px) scale(${this.scale})`;
  }
  /** Zoom in 1 step via public API */
  zoomIn() {
    if (this.isDestroyed || !this.features.zoom) return;
    this.features.zoom.zoomIn();
  }
  /** Zoom out 1 step via public API */
  zoomOut() {
    if (this.isDestroyed || !this.features.zoom) return;
    this.features.zoom.zoomOut();
  }
}
class SvgZoom extends SvgToolbelt {
  constructor(container, config) {
    console.warn("SvgZoom is deprecated and will be removed in a future version. Use SvgToolbelt instead.");
    super(container, config);
  }
}
function initializeSvgToolbelt(selectorOrElements, config = {}) {
  let containers = [];
  if (typeof selectorOrElements === "string") {
    containers = Array.from(
      document.querySelectorAll(selectorOrElements)
    );
  } else if (selectorOrElements instanceof HTMLElement) {
    containers = [selectorOrElements];
  } else if (Array.isArray(selectorOrElements)) {
    containers = selectorOrElements;
  }
  if (containers.length === 0) {
    console.info("SvgToolbelt: No containers found to initialize");
    return;
  }
  containers.forEach((container, idx) => {
    if (container.closest(".svg-toolbelt-wrapper")) {
      return;
    }
    try {
      const svg = container.querySelector("svg");
      if (svg) {
        const wrapper = document.createElement("div");
        wrapper.className = "svg-toolbelt-wrapper";
        container.parentNode.insertBefore(wrapper, container);
        wrapper.appendChild(container);
        const toolbeltInstance = new SvgToolbelt(wrapper, config);
        toolbeltInstance.init();
        wrapper.setAttribute("data-svg-toolbelt-initialized", "true");
        wrapper.svgToolbeltInstance = toolbeltInstance;
        console.info(`SvgToolbelt: Initialized zoom for container #${idx + 1}`);
      } else {
        console.warn(`SvgToolbelt: No <svg> found in container #${idx + 1}`);
      }
    } catch (error) {
      console.error(`SvgToolbelt: Failed to initialize #${idx + 1}:`, error);
    }
  });
}
function initializeSvgZoom(selectorOrElements, config = {}) {
  console.warn("initializeSvgZoom is deprecated and will be removed in a future version. Use initializeSvgToolbelt instead.");
  initializeSvgToolbelt(selectorOrElements, config);
}
export {
  ControlsFeature,
  DblclickResetFeature,
  FullscreenFeature,
  KeyboardFeature,
  NoContextMenuFeature,
  PanFeature,
  SvgEnhancer,
  SvgToolbelt,
  SvgZoom,
  TouchFeature,
  ZoomFeature,
  ZoomLevelIndicatorFeature,
  initializeSvgToolbelt,
  initializeSvgZoom
};
//# sourceMappingURL=svg-toolbelt.esm.js.map
