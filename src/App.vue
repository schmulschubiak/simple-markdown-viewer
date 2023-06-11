<template>
  <div v-if="filename" v-html="markdown" class="container">
  </div>
</template>

<script setup lang="ts">
import {listen} from '@tauri-apps/api/event';
import {onMounted, ref} from "vue";
import MarkdownIt from 'markdown-it';
import {invoke} from "@tauri-apps/api";
import {Payload} from "./typings";

const filename = ref();
const markdown = ref();

onMounted(async () => {
  const initialContent: Payload = await invoke("has_initial_content");
  if(initialContent?.content && initialContent?.filename) {
    filename.value = initialContent.filename;
    markdown.value = new MarkdownIt().render(initialContent.content)
  }

  await listen('open-file', (event) => {
    const payload: Payload | undefined = event?.payload as Payload | undefined;

    filename.value = payload?.filename;
    markdown.value = payload?.content && new MarkdownIt().render(payload.content)
  });
});
</script>
