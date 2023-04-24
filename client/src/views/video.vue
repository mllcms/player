<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useVideoItems, type VideoItem } from "@/stores/video";
import { VideoPlayer } from "@videojs-player/vue";
import "video.js/dist/video-js.css";

const route = useRoute();
const router = useRouter();
const name = ref<string>(route.params.name as string);

const store = useVideoItems();
store.request(name.value);

const player = ref();
const handleMounted = (payload: any) => {
  player.value = payload.player;
};

const handleEvent = (player: any) => {
  if (!player) return;
  const index = player.currentTime();
  const len = player.duration() || 9999;
  const progress = { index, len, scale: (index / len) * 100 };
  store.select.progress = progress;
};
</script>
<template>
  <div class="video-container video-container-scale">
    <h2>{{ name }}</h2>
    <button @click="router.back()">返回</button>

    <video-player
      autoplay
      class="video-player vjs-big-play-centered"
      :src="'/api' + store.select.path"
      crossorigin="anonymous"
      playsinline
      controls
      :volume="0.6"
      :playback-rates="[0.5, 1.0, 1.5, 2.0, 2.5, 3]"
      @mounted="handleMounted"
      @timeupdate="handleEvent(player)"
    />

    <nav>
      <div
        class="item"
        v-for="item in store.data"
        @click="store.select = item"
        :class="{ active: item == store.select }"
        :style="{
          backgroundImage: `linear-gradient(to right, #449DFC ${item.progress.scale}%,transparent 0%)`,
        }"
      >
        <p>{{ item.name }}</p>
      </div>
    </nav>
    <h3>{{ store.select.name }}</h3>
  </div>
</template>

<style scoped>
.active {
  color: red;
}

.video-player {
  width: 100%;
  height: 100%;
}

.video-container {
  display: grid;
  grid-template-columns: 1000px 240px;
  grid-template-rows: 40px 600px 40px;
  border-radius: 4px;
  overflow: hidden;
  background-color: #293451;
}

@media screen and (max-width: 1440px) {
  .video-container-scale {
    transform: scale(0.8);
  }
}

@media screen and (max-width: 1000px) {
  .video-container-scale {
    transform: scale(0.6);
  }
}

.video-container button {
  background-color: white;
  border: none;
  margin: 10px 10px 0;
  border-radius: 4px;
  cursor: pointer;
}

.video-container h2,
h3 {
  text-align: center;
  background-color: #293451;
  color: white;
  line-height: 40px;
  padding: 0 20px;
  text-overflow: ellipsis;
  overflow: hidden;
  white-space: nowrap;
}

.video-container nav {
  display: grid;
  row-gap: 10px;
  margin: 10px 0 10px 10px;
  grid-area: span 2;
  cursor: pointer;
  overflow-y: scroll;
}

.video-container nav::-webkit-scrollbar {
  width: 10px;
  background-color: transparent;
}

.video-container nav::-webkit-scrollbar-thumb {
  background-color: white;
  border-radius: 4px;
  border: 2px solid #293451;
}

.video-container nav .item {
  height: 40px;
  display: grid;
  align-items: center;
  background-color: white;
  border-radius: 4px;
  padding: 0 10px;
}

.video-container nav .item p {
  font-size: 14px;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  overflow: hidden;
}
</style>
