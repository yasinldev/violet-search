function scrap_ddg_suggestions(value) {
    value.forEach(option => {
        const optionElement = document.createElement('a');
        optionElement.href = "localhost:8000/search?q=" + encodeURIComponent(option.phrase);
        optionElement.className = "dropdown-item";
        optionElement.textContent = option.phrase;

        dropdown.appendChild(optionElement);
    });
}

function scrap_artado_suggestions(value) {
    let i = 0;
    value.forEach(option => {
        if (i++ > 7) return;
        const optionElement = document.createElement('a');
        optionElement.href = "localhost:8000/search?q=" + encodeURIComponent(option.Keyword); // URL'e uygun kodlama
        optionElement.className = "dropdown-item";
        optionElement.textContent = option.Keyword;

        dropdown.appendChild(optionElement);
    });

}


function scrap_google_suggestions(value) {
    let i = 0;
    value.keywords[1].forEach(option => {
        if (i++ > 7) return;
        const optionElement = document.createElement('a');
        optionElement.href = "localhost:8000/search?q=" + encodeURIComponent(option); // URL'e uygun kodlama
        optionElement.className = "dropdown-item";
        optionElement.textContent = option;

        dropdown.appendChild(optionElement);
    });
}
