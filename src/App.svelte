<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import { register } from "@tauri-apps/api/globalShortcut";
  import { onMount } from "svelte";

  const onKeyUp = async (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      await appWindow.hide();
    }
  };

  onMount(async () => {
    await appWindow.center();
    await register("CommandOrControl+Shift+A", async () => {
      const visible = await appWindow.isVisible();
      console.log("visibility_state=" + visible);
      if (visible) {
        appWindow.hide();
      } else {
        appWindow.show();
        appWindow.setFocus();
      }
    });
  });
</script>

<main class="h-screen w-screen grid place-items-center" on:keyup={onKeyUp}>
  <div class="text-center w-60">
    <img class="w-30 h-30" src="icon.png" alt="Tauri" />
    <h1 class="text-2xl">Welcome!</h1>
    <p>
      This is Tauri shell. The content you are seeing is coming from embedded
      Svelte app.
    </p>
    <hr class="mt-2 mb-2" />
    <p>For styling Tailwind CSS is configured.</p>
  </div>
</main>

<style lang="postcss">
  @tailwind base;
  @tailwind utilities;
  @tailwind components;

  * {
    background: transparent;
    color: white;
  }
</style>
