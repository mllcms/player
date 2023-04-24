<script setup lang="ts">
import { useVideoInfo } from "@/stores/video";
import { useRouter } from "vue-router";

const router = useRouter();
const store = useVideoInfo();
store.request();

const click = (name: string) => {
  router.push({ name: "video", params: { name } });
};
</script>
<template>
  <div class="list-container list-columns">
    <div class="list-info" v-for="item in store.data" @click="click(item.name)">
      <img :src="item.icon || '/icon/icon.png'" alt="" />
      <div class="title">
        <p>{{ item.name }}</p>
      </div>
    </div>
  </div>
</template>

<style>
.list-container {
  display: grid;
  grid-template-rows: repeat(auto-fill, 240px);
  gap: 40px;
  margin: 4vh 0;
}

.list-columns {
  grid-template-columns: repeat(4, 300px);
}

@media screen and (max-width: 1440px) {
  .list-columns {
    grid-template-columns: repeat(3, 300px);
  }
}

@media screen and (max-width: 1000px) {
  .list-columns {
    grid-template-columns: repeat(2, 300px);
  }
}

@media screen and (max-width: 500px) {
  .list-columns {
    grid-template-columns: repeat(1, 300px);
  }
}

.list-info {
  display: grid;
  grid-template-rows: 168px 1fr;
  background-color: white;
  border-radius: 10px;
  overflow: hidden;
  transition: all 0.2s;
  cursor: pointer;
}

.list-info:hover {
  transform: translateY(-4px);
  box-shadow: 0 0 20px 10px #d2d2d2;
}

.list-info img {
  width: 100%;
  height: 168px;
  object-fit: fill;
}

.list-info .title {
  display: grid;
  justify-content: center;
  align-items: center;
  padding: 10px;
  box-sizing: border-box;
}

.list-info p {
  text-align: center;
  font-size: 20px;
  font-weight: bold;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  overflow: hidden;
}
</style>
