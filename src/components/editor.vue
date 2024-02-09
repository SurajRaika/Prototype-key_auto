<template>
  <button :hidden="record" @click="start_recording(true)" class=" bg-green-500 ">Record</button>
  <button :hidden="!record" @click="start_recording(false)" class=" bg-red-500 ">Stop</button>

  <div class="flex">

    <div ref="grid" class=" border" id="grid">
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
      <div class="column"></div>
    </div>
    <textarea ref="test_section" class=" border overflow-scroll w-[50%]  bg-black" placeholder="Editor">

    </textarea>
  </div>
</template>
  
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Event, listen } from '@tauri-apps/api/event';

import { ref } from 'vue';

const grid = ref<HTMLElement | null>(null);
const test_section = ref<HTMLTextAreaElement | null>(null);
const record = ref<boolean>(false);


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

async function start_recording(bool: boolean) {
  record.value = bool;
if (bool) { 
  const ans = await invoke("start_record");
  console.log(ans);
}else{
  const ans = await invoke("stop_record");
  console.log(ans);

}

}

async function Intialise() {

  await listen<String>('updateCounter', (event: Event<String>) => {


    const keyPresses: KeyPress[] = JSON.parse(event.payload.toString());
    if (grid.value && test_section.value) {
      const han_num: number = keyPresses.length - 1;
      const gridColumns: string[] = Array.from({ length: 7 }, (_, index) => index === 0 ? '1fr' : index <= han_num ? '1fr' : '0fr');
      grid.value.style.gridTemplateColumns = gridColumns.join(' ');
      console.log(keyPresses);

      if (record.value) {
        test_section.value.value = test_section.value.value + "\n" + JSON.stringify(keyPresses);
      }
    }

  })
}
Intialise()
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
  
