<template>
  <div>
    <div id="reader"></div>
  </div>
</template>

<script>
import ePub from 'epubjs';

export default {
  mounted: async function () {
    let url = `/api/v1/books/${this.$route.query.book}/files/epub`
    let book = ePub(url, {
      openAs: "epub"
    })

    // // Navigation loaded
    // book.loaded.navigation.then(function (toc) {
    //   console.log(toc);
    // });

    var rendition = book.renderTo("reader", { width: 600, height: 400 });
    // var rendition = book.renderTo("area", { flow: "scrolled-doc", width: 600, height: 400 })
    var displayed = rendition.display(0);

    displayed.then(function (renderer) {
      // -- do stuff
      console.log("rendered")
    });
  }
}
</script>