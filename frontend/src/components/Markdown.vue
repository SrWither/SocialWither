<template>
  <div>
    <div v-if="markdownContent" v-html="renderedMarkdown" id="content" @click="handleClick"></div>
    <div v-else>No content</div>
  </div>
</template>

<script setup lang="ts">
import MarkdownIt from "markdown-it";
import { full as emoji } from "markdown-it-emoji";
import twemoji from "twemoji";

const props = defineProps(["markdownContent"]);
const emits = defineEmits();

const md = MarkdownIt({
  html: true,
  breaks: true,
  linkify: true,
  typographer: true,
}).use(emoji);

const renderedMarkdown = twemoji.parse(md.render(props.markdownContent), {
  folder: "svg",
  ext: ".svg",
});

const handleClick = (event: MouseEvent) => {
  if (event.target instanceof HTMLImageElement) {
    const isTwemoji = event.target.classList.contains("emoji");
    
    if (!isTwemoji) {
      emits("clickImage", event.target.src);
    }
  }
};
</script>
