<script setup>
import { reactive, watch, onMounted } from "vue";
import init, { grayscale } from "/pkg/pixflicker.js";

onMounted(async () => {
  console.log("Mounted pixflicker");
  await init();
});

const images = reactive({
  source: {
    url: "",
  },
  modified: {
    url: "",
  },
});

watch(images.source, (newSource) => {
  const fullUrl = newSource.url;
  const strippedUrl = fullUrl.replace(
    /^data:image\/(jpeg|jpg|png|webp);base64,/,
    ""
  );
  console.log(strippedUrl);

  const modifiedImgUrl = grayscale(strippedUrl);
  images.modified.url = modifiedImgUrl;
  // console.log(modifiedImgUrl);
});

async function onFileChange(event) {
  const files = event.target.files;

  if (!files || !files.length) {
    console.warn("No files selected");
    return;
  }

  for (let i = 0; i < files.length; i++) {
    const currentFile = files[i];

    if (currentFile.size > 100 * 1024 * 1024) {
      alert("Selected a very large file. Maximum file size should be 100mb.");
      return;
    }

    const fr = new FileReader();
    fr.readAsDataURL(currentFile);

    fr.addEventListener("load", () => {
      currentFile.src = fr.result;
      images.source.url = currentFile.src;
    });
  }
}
</script>

<template>
  <div>
    <!-- Images Grid -->
    <div class="images">
      <!-- Left (original image) -->
      <div class="original-image">
        <img
          v-if="images.source.url"
          :src="images.source.url"
          alt="Source Image"
          class="img"
        />
      </div>

      <!-- Right (with filters applied) -->
      <div class="modified-image">
        <img
          v-if="images.modified.url"
          :src="images.modified.url"
          alt="Modified Image"
          class="img"
        />
      </div>
    </div>

    <!-- Upload Button (File Picker) -->
    <input
      type="file"
      @change="onFileChange"
      title="Choose a picture"
      accept="image/*"
    />
  </div>
</template>

<style scoped>
.images {
  width: 1200px;
  height: 600px;
  margin-bottom: 1rem;
  display: flex;
  justify-content: space-between;
  gap: 3rem;
}

.original-image,
.modified-image {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

/* .original-image {
  border: 2px dotted yellow;
}

.modified-image {
  border: 2px dotted cyan;
} */

.img {
  width: 100%;
}
</style>
