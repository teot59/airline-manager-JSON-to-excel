<script setup lang="ts">
import { ref } from "vue";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';

const greetMsg = ref("");
//const fileContent = ref<string>("");
const switchValue = ref<string>("pax"); // Default to "pax"

async function openFileExplorer() {
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'JSON', extensions: ['json','txt'] }]
    });
    
    if (selected === null || Array.isArray(selected)) {
      greetMsg.value = "No file selected";
      return;
    }
    
    const content = await readTextFile(selected);
    //fileContent.value = content;
    
    
    // Send the JSON to Rust with switch value
    const filepath : string = await invoke('process_json', { 
      jsonString: content, 
      selected: selected,
      mode: switchValue.value 
    });
    greetMsg.value = `File saved: ${filepath.split('/').pop()}`;
    
  } catch (error) {
    console.error("Error:", error);
    greetMsg.value = `Error: ${error instanceof Error ? error.message : String(error)}`;
  }
}

function toggleSwitch() {
  switchValue.value = switchValue.value === "pax" ? "cargo" : "pax";
}
</script>

<template>
  <main class="container">

    <!-- File Selection Button and Switch -->
    <div class="file-selector">
      <div class="controls-row">
        <button @click="openFileExplorer" class="select-button square-button">
          📁
        </button>
        
        <div class="switch-container">
          <div class="switch" @click="toggleSwitch" :class="{ 'switched': switchValue === 'cargo' }">
            <div class="switch-slider"></div>
          </div>
        </div>
      </div>
      
      <!-- Switch output display -->
      <div class="switch-output">
        {{ switchValue }}
      </div>
    </div>

    <p>{{ greetMsg }}</p>

    <!--div v-if="fileContent" class="content-display">
      <h3>File Content:</h3>
      <pre>{{ fileContent }}</pre>
    </div-->
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

.switch-container {
  display: flex;
  align-items: center;
}

.switch {
  width: 120px;
  height: 70px;
  background-color: #ccc;
  border-radius: 8px;
  position: relative;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.switch.switched {
  background-color: #396cd8;
}

.switch-slider {
  width: 36px;
  height: 66px;
  background-color: white;
  border-radius: 6px;
  position: absolute;
  top: 2px;
  left: 3px;
  transition: transform 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.switch.switched .switch-slider {
  transform: translateX(78px);
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
  
  .switch {
    background-color: #555;
  }
  
  .switch.switched {
    background-color: #24c8db;
  }
  
  .switch-output {
    background-color: #333;
    border-color: #555;
    color: #f6f6f6;
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

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
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
}</style>