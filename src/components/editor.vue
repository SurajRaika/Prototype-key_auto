<template>
    <div ref="grid" id="grid">
        <div class="column"></div>
        <div class="column"></div>
        <div class="column"></div>
        <div class="column"></div>
        <div class="column"></div>
        <div class="column"></div>
        <div class="column"></div>
    </div>
</template>
  
<script setup lang="ts">
import { Event, listen } from '@tauri-apps/api/event';
import { ref } from 'vue';

const grid=ref<HTMLElement | null>(null);

async function sta() {
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
  await listen<String>('updateCounter', (event:Event<String>) => {
    const keyPresses: KeyPress[] = JSON.parse(event.payload.toString());
    console.log(keyPresses);
    if (grid.value) {
        const han_num:number=keyPresses.length-1;
        const gridColumns:string[] = Array.from({ length: 7 }, (_, index) => index === 0 ? '1fr' : index <= han_num ? '1fr' : '0fr');
        grid.value.style.gridTemplateColumns=gridColumns.join(' ');;
    }

  })
}
sta()

</script>
  
<style scoped>
#grid {
    height: 100vh;
    transition: 100ms;
    display: grid;
    grid-template-columns: 1fr 0fr 0fr 0fr 0fr 0fr 0fr;
}



.column {
    outline: 1.5px solid rgb(0 0 0 / 10%);
}
</style>
  