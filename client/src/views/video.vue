<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useVideoItems } from "@/stores/video";
const route = useRoute();
const router = useRouter();
const name = ref<string>(route.params.name as string);
const store = useVideoItems();

const video = ref<HTMLVideoElement>();
onMounted(async () => {
  await store.request(name.value);
  document.querySelector("nav div.active")?.scrollIntoView();
});

// 时间更新
const timeUpdate = () => {
  const index = video.value!.currentTime;
  if (!video.value || index < 1) return; // 防止初始加载顶掉缓存加载

  const len = video.value.duration || 9999;
  const progress = { index, len, scale: (index / len) * 100 };
  store.select.progress = progress;
};

// 播放结束放下一集
const next = () => {
  const index = store.data.findIndex((item) => store.select == item);
  const item = store.data[index + 1];
  if (item) {
    store.select = item;
  }
};

// 加载播放进度
const loadProgress = () => {
  if (video.value) {
    video.value.currentTime = store.select.progress.index;
  }
};
</script>
<template>
  <div class="video-container video-container-scale">
    <h2>{{ name }}</h2>
    <button @click="router.back()">返回</button>

    <video
      ref="video"
      class="video"
      :src="'/api' + store.select.path"
      @ended="next"
      @timeupdate="timeUpdate"
      @loadstart="loadProgress"
      controls
      autoplay
      preload="auto"
    >
      <select name="multiplier" id="multiplier">
        <option value="1.75">1.75</option>
      </select>
    </video>

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

<style>
.active {
  color: red;
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
.video {
  width: 100%;
  height: 100%;
  object-fit: fill;
}
</style>
