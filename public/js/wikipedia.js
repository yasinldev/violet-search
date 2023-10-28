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

            for (let i = 1; i < data[1].length; i++) {
                const title = data[1][i];
                const href = data[3][i];
                const div = document.createElement('a');
                div.innerHTML = title;
                div.href = href;
                wikipedia_titles.appendChild(div);
                if(i < data[1].length - 1)
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

    const url = `https://${language}.wikipedia.org/w/api.php?origin=*&action=query&list=search&srsearch=${title}&utf8=&format=json&prop=extracts&exintro&explaintext&titles=${title}&imageinfo&iiprop=url`;

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

            const page = data.query.pages;
            const page_id = Object.keys(page)[0];
            let extract = page[page_id].extract;

            let page_subtitle = "";

            extract = extract.replace(/\([^)]*\)/g, '').trim();

            if (extract.length > 500) {
                extract = extract.substring(0, 500) + '...';
            }

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
