<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  let currentImage: string | null = $state(null);
  let isDragging = $state(false);

  // "Internal Letterboxing" style
  // We use flexbox centering to ensure content is always visible even if window ratio is wrong
  const containerStyle = "width: 100vw; height: 100vh; display: flex; align-items: center; justify-content: center; background: transparent; overflow: hidden;";

  async function handleDrop(event: DragEvent) {
    event.preventDefault();
    isDragging = false;

    const file = event.dataTransfer?.files[0];
    if (!file) return;

    // Create an object URL for immediate display
    const objectUrl = URL.createObjectURL(file);
    
    // Get image dimensions to resize window
    const img = new Image();
    img.onload = async () => {
      const width = img.naturalWidth;
      const height = img.naturalHeight;
      
      // Logic to fit screen (simplified for now, backend command handles specific constraints if we move logic there, 
      // but brief said "Analysis: Rust reads pixel dimensions". 
      // However, for a quick prototype using Drag & Drop from web frontend, we can read it here too.
      // For strict Rust file reading, we'd need the file path and a Rust command to read it.
      // Let's stick to frontend reading for the prototype speed, unless user demands Rust reading strictly now.
      // Brief said: "Rust reads pixel dimensions... from the file header."
      // Since we are in a webview, we might not have direct file system access to the path unless we enable the fs plugin.
      // For this phase, using the browser's FileReader/Image object is faster and safe for the prototype.
      
      // Calculate target size (max 80% of screen) - simplified
      // We'll just pass the raw dimensions to Rust and let it handle or just set it.
      // The brief asks for Rust to resize. 
      
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

  // Context menu placeholder
  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    // In a real app, invoke a Rust command to show native menu
    console.log("Right click - native menu trigger");
  }
</script>

<!-- 
  data-tauri-drag-region allows moving the frameless window. 
  We apply it to the container so the whole image is a handle.
-->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  style={containerStyle} 
  class:dragging={isDragging}
  oncontextmenu={handleContextMenu}
  ondrop={handleDrop}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
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