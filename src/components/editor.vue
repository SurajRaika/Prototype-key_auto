<template>
  <button :hidden="!record" @click="startRecording(false)" class="bg-red-500">Stop</button>
  <button :hidden="record" @click="startRecording(true)" class="bg-green-500">Record</button>
  <input type="checkbox" v-model="isEventKey" name="Event_key">
  {{ recordingMessage }}
  <button @click="save">Save</button>
  <input type="text" v-model="name" placeholder="Enter Name " name="Keys_name" id="">
  <div class="flex">
    <div ref="grid" class="border" id="grid">
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
    </div>
    <textarea ref="testSection" class="border overflow-scroll w-[50%] bg-black text-white" placeholder="Editor"></textarea>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { writeTextFile , createDir , readTextFile, BaseDirectory } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import { Event, listen } from '@tauri-apps/api/event';

const isEventKey = ref<boolean>(false);
const grid = ref<HTMLElement | null>(null);
const testSection = ref<HTMLTextAreaElement | null>(null);
const record = ref<boolean>(false);
const Event_recording=ref<KeyPress[][]>([]);
const name=ref<String>("");


const recordingMessage = computed(() => {
  return isEventKey.value ? 'Event Key Recording' : 'Record Key Recording';
});

interface Keys_with_id{
  id:number;
  name:String;
  Keys:KeyPress[][]
}

interface KeyPress {
  key: string;
  duration: {
    secs: number;
    nanos: number;
  };
  start_time: {
    secs: number;
    nanos: number;
  };
}

async function startRecording(isStart: boolean) {
  record.value = isStart;
  const action = isStart ? "start_record" : "stop_record";
  const ans = await invoke(action);
  console.log(ans);
}

async function save() {
  const fileName = isEventKey.value ? 'EventKeys.json' : 'ActionKeys.json';
  let id = 0;
  
  // Read the file to check if it exists
  try {
    await readTextFile(fileName, { dir: BaseDirectory.AppData });
  } catch (error) {
    // If file not available, create it
    await createDir('users', { dir: BaseDirectory.AppData, recursive: true });
    await writeTextFile(fileName, '[]', { dir: BaseDirectory.AppData });
  }

  // Read the content of the file to generate a new unique ID
  const fileContents = await readTextFile(fileName, { dir: BaseDirectory.AppData });
  const keysWithIdArray: Keys_with_id[] = JSON.parse(fileContents);
  
  // Find the maximum ID in the existing data
  if (keysWithIdArray.length > 0) {
    id = Math.max(...keysWithIdArray.map(item => item.id)) + 1;
  } else {
    id = 1; // Set initial ID to 1 if file is empty
  }
  
  // Prepare data to save
  const keysWithId: Keys_with_id = { Keys: Event_recording.value, id: id , name:name.value };
  keysWithIdArray.push(keysWithId);
  
  // Write updated data back to the file
  await writeTextFile(fileName, JSON.stringify(keysWithIdArray), { dir: BaseDirectory.AppData });
}

async function initialize() {
  await listen<String>('updateCounter', (event: Event<String>) => {
    const keyPresses: KeyPress[] = JSON.parse(event.payload.toString());
    if (grid.value && testSection.value) {
      const hanNum: number = keyPresses.length - 1;
      const gridColumns: string[] = Array.from({ length: 7 }, (_, index) => index === 0 ? '1fr' : index <= hanNum ? '1fr' : '0fr');
      grid.value.style.gridTemplateColumns = gridColumns.join(' ');

      if (record.value) {
        Event_recording.value.push(keyPresses);
        testSection.value.value += "\n" + JSON.stringify(keyPresses);
      }
    }
  });
}

initialize();
</script>

<style scoped>
#grid {
  width: 50%;
  height: 100vh;
  transition: 100ms;
  display: grid;
  grid-template-columns: 1fr 0fr 0fr 0fr 0fr 0fr 0fr;
}

.column {
  outline: 1.5px solid rgb(0 0 0 / 10%);
}
</style>
