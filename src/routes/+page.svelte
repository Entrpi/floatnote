<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let currentImage: string | null = $state(null);
  let isDragging = $state(false);
  let opacity = $state(1.0);
  let isInteractive = $state(true);

  const appWindow = getCurrentWindow();

  let keybindings = $state({
      zoomIn: { keys: ["=", "+"], meta: false },
      zoomOut: { keys: ["-", "_"], meta: false },
      toggleInteract: { keys: ["i"], meta: true },
  });

  let containerStyle = $derived(`width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; background: transparent; overflow: hidden; opacity: ${opacity}; position: relative;`);

  onMount(async () => {
    await listen<number>("set_opacity", (event) => {
        opacity = event.payload;
    });
    await listen("toggle_interaction", () => {
        isInteractive = !isInteractive;
    });

    // Tauri File Drop Events
    await listen<{ paths: string[] }>("tauri://drag-drop", async (event) => {
        if (!isInteractive) return;
        isDragging = false;
        const path = event.payload.paths[0];
        if (path) {
            try {
                // Rust analyzes dimensions and resizes window
                await invoke("open_image", { path });
                // Convert path to asset URL for display
                currentImage = convertFileSrc(path);
            } catch (e) {
                console.error("Failed to open image:", e);
            }
        }
    });

    await listen("tauri://drag-enter", () => {
        if (isInteractive) isDragging = true;
    });

    await listen("tauri://drag-leave", () => {
        isDragging = false;
    });
  });

  // Removed HTML ondrop/dragover for file logic, kept visual feedback logic via Tauri events
  // But we can keep HTML handlers to prevent default browser behavior (opening file in viewport directly)
  function preventDefault(event: DragEvent) {
      event.preventDefault();
  }

  async function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    await invoke("show_context_menu", { isInteractive });
  }

  async function zoom(factor: number) {
    await invoke("zoom_window", { factor });
  }

  function handleWheel(event: WheelEvent) {
    if (!currentImage || !isInteractive) return;
    const factor = event.deltaY < 0 ? 1.05 : 0.95;
    zoom(factor);
  }

  function isMatch(event: KeyboardEvent, action: keyof typeof keybindings) {
      const config = keybindings[action];
      const keyMatch = config.keys.includes(event.key);
      const metaMatch = config.meta ? (event.ctrlKey || event.metaKey) : true;
      return keyMatch && metaMatch;
  }

  function handleKeydown(event: KeyboardEvent) {
      if (!currentImage) return;
      
      if (isInteractive) {
        if (isMatch(event, "zoomIn")) {
            zoom(1.05);
        } else if (isMatch(event, "zoomOut")) {
            zoom(0.95);
        }
      }

      if (isMatch(event, "toggleInteract")) {
          isInteractive = !isInteractive;
      }
  }

  function startResize(direction: string) {
      if (!isInteractive) return;
      appWindow.startResizeDragging(direction as any);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  style={containerStyle} 
  class:dragging={isDragging}
  class:interactive={isInteractive}
  oncontextmenu={handleContextMenu}
  ondrop={preventDefault}
  ondragover={preventDefault}
  onwheel={handleWheel}
  data-tauri-drag-region={isInteractive ? "" : null}
>
  {#if currentImage}
    <img 
      src={currentImage} 
      alt="Reference" 
      style="max-width: 100%; max-height: 100%; pointer-events: none;" 
    />
  {:else}
    <div style="text-align: center; color: #888; pointer-events: none;">
      <p>Drop Image Here</p>
    </div>
  {/if}

  {#if isInteractive}
      <div class="resize-handle n" onmousedown={() => startResize('Top')}></div>
      <div class="resize-handle s" onmousedown={() => startResize('Bottom')}></div>
      <div class="resize-handle e" onmousedown={() => startResize('Right')}></div>
      <div class="resize-handle w" onmousedown={() => startResize('Left')}></div>
      <div class="resize-handle ne" onmousedown={() => startResize('TopRight')}></div>
      <div class="resize-handle nw" onmousedown={() => startResize('TopLeft')}></div>
      <div class="resize-handle se" onmousedown={() => startResize('BottomRight')}></div>
      <div class="resize-handle sw" onmousedown={() => startResize('BottomLeft')}></div>
  {/if}
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
    user-select: none;
    overflow: hidden;
  }

  .dragging {
    background: rgba(255, 255, 255, 0.1) !important;
  }

  .resize-handle {
      position: absolute;
      z-index: 1000;
      background: rgba(100, 149, 237, 0.5);
      opacity: 0;
      transition: opacity 0.2s ease-in-out;
  }

  .interactive:hover .resize-handle {
      opacity: 0.1;
  }
  .resize-handle:hover {
      opacity: 0.6 !important;
  }

  .n, .s { height: 10px; width: 100%; left: 0; cursor: ns-resize; }
  .e, .w { width: 10px; height: 100%; top: 0; cursor: ew-resize; }
  .n { top: 0; }
  .s { bottom: 0; }
  .e { right: 0; }
  .w { left: 0; }

  .ne, .nw, .se, .sw { width: 20px; height: 20px; }
  .ne { top: 0; right: 0; cursor: nesw-resize; }
  .nw { top: 0; left: 0; cursor: nwse-resize; }
  .se { bottom: 0; right: 0; cursor: nwse-resize; }
  .sw { bottom: 0; left: 0; cursor: nesw-resize; }
</style>