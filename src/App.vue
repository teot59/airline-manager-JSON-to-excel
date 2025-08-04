<script setup lang="ts">
import { ref, nextTick, onMounted } from "vue";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";


const errorPrefix = ref("");
const errorMsg = ref("");
//const fileContent = ref<string>("");
const switchValue = ref<string>("pax"); // Default to "pax"
const copyButton = ref(false);
const showErrorBox = ref(false);
const containerRef = ref(null);
const fileSelectorRef = ref(null);
const messageContainerRef = ref(null);

onMounted(async () => {
  try {
    await resizeWindowToContent();

  } catch (error) {
    console.error('Error getting window size:', error);
  }
});

/*function getElementHeight(element: HTMLElement | null): number {
  return element?.offsetHeight || 0;
}*/

async function getAllElementHeights() {
  await nextTick();
  await new Promise(resolve => setTimeout(resolve, 10));
  var body = document.body,
      html = document.documentElement;

  var height = Math.max( body.scrollHeight, body.offsetHeight, html.offsetHeight);
  //Math.max( body.scrollHeight, body.offsetHeight, html.clientHeight, html.scrollHeight, html.offsetHeight );
  /*const heights = {
    container: getElementHeight(containerRef.value),
    fileSelector: getElementHeight(fileSelectorRef.value),
    messageContainer: getElementHeight(messageContainerRef.value),
    total: 0
  };
  
  heights.total = heights.container;
  console.log('Element heights:', heights);
  return heights;*/
  return height;
}

async function resizeWindowToContent() {
  let defaultWidth = (await getCurrentWindow().innerSize()).width;
  let height = await getAllElementHeights();
  await getCurrentWindow().setSize(new LogicalSize(defaultWidth, Math.max(height + 20, 200)));
}

async function openFileExplorer() {
  
  copyButton.value = false;
  showErrorBox.value = false;
  await resizeWindowToContent();
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'JSON', extensions: ['json','txt'] }]
    });
    
    if (selected === null || Array.isArray(selected)) {
      showErrorBox.value = true;
      errorMsg.value = "No file selected";
      await resizeWindowToContent();
      
      return;
    }
    
    const content = await readTextFile(selected);
    
    
    // Send the JSON to Rust with switch value
    const filepath : string = await invoke('process_json', { 
      jsonString: content, 
      selected: selected,
      mode: switchValue.value 
    });
    copyButton.value = true;
    showErrorBox.value = true;
    errorPrefix.value = `File saved: `
    errorMsg.value = `${filepath.split('/').pop()}`;
    await resizeWindowToContent();
    
  } catch (error) {
    console.error("Error:", error);
    copyButton.value = true;
    showErrorBox.value = true;
    errorPrefix.value = `Error: `
    errorMsg.value = `${error instanceof Error ? error.message : String(error)}`;
    await resizeWindowToContent();
  }
}

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(errorMsg.value);
  } catch (error) {
    console.error('Failed to copy:', error);
  }
}

function selectMode(mode: string) {
  switchValue.value = mode;
}

</script>

<template>
  <main class="container" ref="containerRef">

    <div class="file-selector" ref="fileSelectorRef">
      <div class="controls-row">
        <button @click="openFileExplorer" class="select-button square-button">
          <div>
            <img src="/src-tauri/icons/folder_144x144.png" alt="Submit"></img>
          </div>

            

        </button>
        <div class="checkbox-container">
          <div class="checkbox-row">
            <input type="checkbox" id="pax_check" value="pax" :checked="switchValue === 'pax'" @change="selectMode('pax')">
            <label for="pax_check">pax</label>
          </div>
          <div class="checkbox-row">
            <input type="checkbox" id="cargo_check" value="cargo" :checked="switchValue === 'cargo'" @change="selectMode('cargo')">
            <label for="cargo_check">cargo</label>
          </div>
        </div>

      </div>
      <!--div class="row"-->
        
      <!--/div-->
      <!--div class="switch-output">
        {{ switchValue }}
      </div-->
    </div>
    <div v-if="showErrorBox" class="message-container" ref="messageContainerRef">
      <div class="message-content">
        <div class="message-text">
          <p class="message-prefix">{{ errorPrefix }}</p>
          <p class="message-body">{{ errorMsg }}</p>
        </div>
        <button v-if="copyButton" @click="copyToClipboard" class="copy-button">
          <img src="/src-tauri/icons/clipboard_96x96.png" alt="Submit"></img>
        </button>
      </div>
    </div>
    

  </main>
</template>

<style scoped>
.content-display {
  margin-top: 20px;
  text-align: left;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 8px;
  max-width: 800px;
  margin-left: auto;
  margin-right: auto;
  overflow-x: auto;
}

pre {
  white-space: pre-wrap;
  word-wrap: break-word;
}

.checkbox-container {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.checkbox-row {
  margin: 1px 0;
}

.checkbox-row input[type="checkbox"] {
  display: none;
}

.checkbox-row label {
  display: inline-block;
  padding: 12px 24px;
  background-color: #ffffff;
  border: 1px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 1em;
  font-weight: 500;
  color: #0f0f0f;
  transition: border-color 0.25s;
  user-select: none;
  width: 60%;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

.checkbox-row label:hover {
  border-color: #396cd8;
}

.checkbox-row input[type="checkbox"]:checked + label {
  border-color: #396cd8;
  color: #396cd8;
  background-color: #e8e8e8;
}

@media (prefers-color-scheme: dark) {
  .checkbox-row label {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  .checkbox-row label:hover {
    border-color: #24c8db;
  }

  .checkbox-row input[type="checkbox"]:checked + label {
    border-color: #24c8db;
    color: #24c8db;
    background-color: #0f0f0f69;
  }
}

.checkbox-row label:active {
    transform: translateY(2px);
}

.file-selector {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  margin: 20px 0;
}

.controls-row {
  display: flex;
  align-items: center;
  gap: 20px;
}

.select-button {
  padding: 12px 24px;
  font-size: 1.1em;
  background-color: #396cd8;
  color: white;
  border: none;
  border-radius: 8px;
  transition: background-color 0.3s ease;
}

.square-button {
  width: 100px;
  height: 100px;
  padding: 0;
  font-size: 2.5em;
  display: flex;
  align-items: center;
  justify-content: center;
}

.select-button:hover {
  background-color: #2a5bc7;
}
.square-button img {
  max-width: 70%;
  max-height: 70%;
  object-fit: contain;
  padding-top: 10%;
}

.switch-output {
  padding: 8px 16px;
  background-color: #f0f0f0;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-weight: 500;
  min-width: 60px;
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  .content-display {
    background-color: #1a1a1a;
  }
  
  .select-button {
    background-color: #24c8db;
    color: #0f0f0f;
    font-weight: bold;
  }
  
  .select-button:hover {
    background-color: #1eb8c9;
  }
  
  .switch-output {
    background-color: #333;
    border-color: #555;
    color: #f6f6f6;
  }
}

.message-container {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.message-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  background-color: #ffffff;
  border: 1px solid transparent;
  border-radius: 8px;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  transition: border-color 0.25s;
}

.message-text {
  flex: 1;
  text-align: left;
  cursor: pointer;
  user-select: none;
  min-width: 0;
  overflow-wrap: anywhere;
}

.message-prefix {
  margin: 0;
  font-weight: 600;
  color: #0f0f0f;
}

.message-body {
  margin: 4px 0 0 0;
  word-wrap: break-word;
  overflow-wrap: break-word;
  hyphens: auto;
  color: #0f0f0f;
  min-width: 0;
}

.copy-button {
  width: 40px;
  height: 40px;
  padding: 0;
  margin-left: 12px;
  font-size: 1.2em;
  background-color: #396cd8;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.3s ease, transform 0.1s ease;
  flex-shrink: 0;
}
.copy-button img {
  max-width: 65%;
  max-height: 65%;
  object-fit: contain;
 
}

.copy-button:hover {
  background-color: #2a5bc7;
}

.copy-button:active {
  transform: translateY(1px);
}
@media (prefers-color-scheme: dark) {
  .message-content {
    background-color: #0f0f0f98;
  }
  

  .message-prefix,
  .message-body {
    color: #ffffff;
  }

  .copy-button {
    background-color: #24c8db;
    color: #0f0f0f;
    font-weight: bold;
  }
  
  .copy-button:hover {
    background-color: #1eb8c9;
  }
}




</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>