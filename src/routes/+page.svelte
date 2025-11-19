<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let currentImage: string | null = $state(null);
  let isDragging = $state(false);
  let opacity = $state(1.0);

  // We use flexbox centering to ensure content is always visible
  // We bind opacity to the container
  let containerStyle = $derived(`width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; background: transparent; overflow: hidden; opacity: ${opacity};`);

  onMount(async () => {
    // Listen for backend opacity events
    await listen<number>("set_opacity", (event) => {
        opacity = event.payload;
    });
  });

  async function handleDrop(event: DragEvent) {
    event.preventDefault();
    isDragging = false;

    const file = event.dataTransfer?.files[0];
    if (!file) return;

    const objectUrl = URL.createObjectURL(file);
    
    const img = new Image();
    img.onload = async () => {
      const width = img.naturalWidth;
      const height = img.naturalHeight;
      await invoke("resize_window", { width, height });
      currentImage = objectUrl;
    };
    img.src = objectUrl;
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    isDragging = true;
  }

  function handleDragLeave() {
    isDragging = false;
  }

  async function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    // Trigger native context menu
    await invoke("show_context_menu");
  }

  async function zoom(factor: number) {
    await invoke("zoom_window", { factor });
  }

  function handleWheel(event: WheelEvent) {
    if (!currentImage) return;
    // event.deltaY < 0 is scrolling up (zoom in)
    const factor = event.deltaY < 0 ? 1.05 : 0.95;
    zoom(factor);
  }

  function handleKeydown(event: KeyboardEvent) {
      if (!currentImage) return;
      if (event.key === "=" || event.key === "+") {
          zoom(1.05);
      } else if (event.key === "-") {
          zoom(0.95);
      }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  style={containerStyle} 
  class:dragging={isDragging}
  oncontextmenu={handleContextMenu}
  ondrop={handleDrop}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  onwheel={handleWheel}
  data-tauri-drag-region
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
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent; /* Important for transparent window */
    user-select: none;
  }

  .dragging {
    background: rgba(255, 255, 255, 0.1) !important;
  }
</style>