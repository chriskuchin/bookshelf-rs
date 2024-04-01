import("./css/read.scss");

import ePub from "epubjs";


const params = URLSearchParams && new URLSearchParams(document.location.search.substring(1));
const url = params?.get("url") && decodeURIComponent(params.get("url"));
const currentSectionIndex = (params?.get("loc")) ? params.get("loc") : undefined;

// Load the opf
window.book = ePub(url || "/api/v1/books/3/files/epub/", { openAs: "epub" });
const rendition = book.renderTo("viewer", {
    manager: "continuous",
    flow: "paginated",
    width: "100%",
    height: "100%",
    spread: "never",
});

const displayed = rendition.display(currentSectionIndex);


displayed.then((renderer) => {
    // -- do stuff
});

// Navigation loaded
book.loaded.navigation.then((toc) => {
    // console.log(toc);
});

book.ready.then(() => {

    const next = document.getElementById("next");

    next.addEventListener("click", (e) => {
        book.package.metadata.direction === "rtl" ? rendition.prev() : rendition.next();
        e.preventDefault();
    }, false);

    const prev = document.getElementById("prev");
    prev.addEventListener("click", (e) => {
        book.package.metadata.direction === "rtl" ? rendition.next() : rendition.prev();
        e.preventDefault();
    }, false);

    const keyListener = (e) => {

        // Left Key
        if ((e.keyCode || e.which) === 37) {
            book.package.metadata.direction === "rtl" ? rendition.next() : rendition.prev();
        }

        // Right Key
        if ((e.keyCode || e.which) === 39) {
            book.package.metadata.direction === "rtl" ? rendition.prev() : rendition.next();
        }

    };

    rendition.on("keyup", keyListener);
    document.addEventListener("keyup", keyListener, false);

});

rendition.on("selected", (range) => {
    console.log("selected", range);
});

rendition.on("layout", (layout) => {
    const viewer = document.getElementById("viewer");

    if (layout.spread) {
        viewer.classList.remove('single');
    } else {
        viewer.classList.add('single');
    }
});

rendition.on("relocated", (location) => {
    console.log(location);

    const next = book.package.metadata.direction === "rtl" ? document.getElementById("prev") : document.getElementById("next");
    const prev = book.package.metadata.direction === "rtl" ? document.getElementById("next") : document.getElementById("prev");

    if (location.atEnd) {
        next.style.visibility = "hidden";
    } else {
        next.style.visibility = "visible";
    }

    if (location.atStart) {
        prev.style.visibility = "hidden";
    } else {
        prev.style.visibility = "visible";
    }

});

book.loaded.navigation.then((toc) => {
    const $select = document.getElementById("toc")
    const docfrag = document.createDocumentFragment()

    for (chapter of toc) {
        const option = document.createElement("option");
        option.textContent = chapter.label;
        option.ref = chapter.href;

        docfrag.appendChild(option);
    }

    $select.appendChild(docfrag);

    $select.onchange = () => {
        const index = $select.selectedIndex
        const url = $select.options[index].ref
        rendition.display(url);
        return false;
    };

});
