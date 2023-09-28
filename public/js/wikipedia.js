function get_wikipedia_titles(title, language, targetElementId) {
    const wikipedia_titles = document.getElementById(targetElementId);

    const url = `https://${language}.wikipedia.org/w/api.php?origin=*&action=opensearch&search=${title}&limit=5&namespace=0&format=json`;

    fetch(url, {
        method: 'GET',
        mode: 'cors',
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            wikipedia_titles.classList.add('wiki-titles');
            const h3 = document.createElement('h3');
            h3.innerHTML = 'Other Wikipedia titles';
            h3.classList.add('wiki-titles-header');
            h3.classList.add('font-assistant');
            wikipedia_titles.appendChild(h3);

            wikipedia_titles.appendChild(document.createElement('br'));
            for (let i = 1; i < data[1].length; i++) {
                const title = data[1][i];
                const href = data[3][i];
                const div = document.createElement('a');
                div.innerHTML = title;
                div.href = href;
                wikipedia_titles.appendChild(div);
                wikipedia_titles.appendChild(document.createElement('br'));
            }
        })
        .catch(error => {
            console.error('Error:', error);
        }
    );
}

function get_wikipedia_data(title, language, targetElementId) {
    const wikipedia_data = document.getElementById(targetElementId);
    const wikipedia_card = document.getElementById('wikipedia-card');

    const url = `https://${language}.wikipedia.org/w/api.php?origin=*&action=query&prop=extracts&exintro&explaintext&format=json&titles=${title}&imageinfo&iiprop=url`;
    const image_url = `https://${language}.wikipedia.org/w/api.php?origin=*&action=query&prop=pageimages&format=json&piprop=original&titles=${title}`;

    fetch(image_url, {
        method: 'GET',
        mode: 'cors',
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            const page = data.query.pages;
            const page_id = Object.keys(page)[0];
            const image_url = page[page_id].original.source;
            const img = document.createElement('img');
            img.src = image_url;
            img.classList.add('wikipedia-image');
            wikipedia_data.appendChild(img);
        })
        .catch(error => {
            console.error('Error:', error);
        }
    );

    fetch(url, {
        method: 'GET',
        mode: 'cors',
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            wikipedia_data.classList.add('wikipedia-card');
            wikipedia_data.appendChild(document.createElement('br'));

            const page = data.query.pages;
            const page_id = Object.keys(page)[0];
            let extract = page[page_id].extract;

            let page_subtitle = "";

// Remove text within parentheses and trim the result
            extract = extract.replace(/\([^)]*\)/g, '').trim();

            if (extract.length > 500) {
                extract = extract.substring(0, 500) + '...';
            }

// Find the first period (.) to determine the subtitle
            const firstPeriodIndex = extract.indexOf('.');
            if (firstPeriodIndex !== -1) {
                page_subtitle = extract.substring(0, firstPeriodIndex + 1);
            }

            const subtitle = document.createElement('h4');
            subtitle.classList.add('font-assistant');
            subtitle.classList.add('subtitle');
            subtitle.innerHTML = page_subtitle;
            wikipedia_data.appendChild(subtitle);

            const div = document.createElement('div');
            div.innerHTML = extract;
            div.classList.add('font-assistant');
            div.classList.add('suspend-text');
            wikipedia_data.appendChild(div);

        })
        .catch(error => {
            console.error('Error:', error);
        }
    );
}
