import { defineStore } from "pinia";
import axios from "axios";
import { ref, watch } from "vue";
import { getCache, setCache } from "@/utils";

const request = axios.create({
  baseURL: "/api",
  timeout: 1000 * 15,
});

request.interceptors.response.use((res) => {
  return res.data;
});

export interface VideoInfo {
  name: string;
  icon: string;
}
export interface VideoItem {
  name: string;
  path: string;
  progress: {
    index: number;
    len: number;
    scale: number;
  };
}

export const useVideoInfo = defineStore("video-info", () => {
  const data = ref<VideoInfo[]>([]);
  const req = async () => {
    const { data: d } = await request.get("/list");
    const res = d || [];

    data.value = res;
    return res;
  };
  return { data, request: req };
});

const defaults = () => ({
  name: "",
  path: "",
  progress: { index: 0, len: 9999, scale: 0 },
});

export const useVideoItems = defineStore("video", () => {
  const data = ref<VideoItem[]>([]);
  const select = ref<VideoItem>(defaults());

  let _name = "";
  const req = async (name: string) => {
    _name = name;
    const { data: d } = await request.post("/items", { name });
    const res: VideoItem[] = d || [];
    const cache = getCache(name, Array.from({ length: res.length }, defaults));

    for (let i = 0; i < cache.length; i++) {
      res[i].progress = cache[i].progress;
    }

    for (let i = cache.length; i < res.length; i++) {
      res[i].progress = defaults().progress;
    }

    const selectCache = getCache(name + "-select", res[0]);
    select.value = res.find((item) => item.path == selectCache.path) || res[0];

    data.value = res;
    return res;
  };

  // 缓存观看的视频记录
  watch(
    () => select.value,
    () => {
      setCache(_name + "-select", select.value);
      setCache(_name, data.value);
    }
  );

  // 缓存观看的视频进度
  let prevCacheTime = 0;
  watch(
    () => select.value.progress,
    (nv, ov) => {
      if (nv != ov && Date.now() - prevCacheTime < 1000 * 10) return;
      prevCacheTime = Date.now();

      setCache(_name + "-select", select.value);
      setCache(_name, data.value);
    }
  );

  return { data, request: req, select };
});
