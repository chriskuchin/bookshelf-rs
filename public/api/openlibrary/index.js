async function searchLibrary(author, title) {
  const url = 'https://openlibrary.org/search.json'
  // https://openlibrary.org/search.json?title=Oathbringer&author=Brandon+Sanderson

	const filters = new URLSearchParams();

	if (author !== "") {
		filters.append("author", author);
	}

	if (title !== "") {
		filters.append("title", title)
	}

	const res = await fetch(`${url}?${filters.toString()}`);

  if (res.ok) {
    return await res.json()
  }
  throw new Error('failed searching open library')
}

async function getWork(workFragment) {
  const url = `https://openlibrary.org/${workFragment}.json`
}

function getCoverURL(olId, size="M") {
  return `https://covers.openlibrary.org/b/OLID/${olId}-${size}.jpg`
}

export {searchLibrary, getCoverURL}