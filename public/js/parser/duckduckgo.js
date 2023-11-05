function scrap_ddg_web_results(value) {
    const mainElement = document.getElementById("main");

    window.addEventListener("offline" , () => {
        const alertElement = document.getElementById("alerts");
        const alertHTML = `
            <div class="alert alert-warning font-assistant" role="alert">
            Error: You are not connected to the internet. Check your connection and try again.
            </div>
        `;

        alertElement.appendChild(htmlToElement(alertHTML));
    });

    let descArray = JSON.parse(value.desc);
    let titleArray = JSON.parse(value.title);
    let imgArray = JSON.parse(value.img);
    let urlArray = JSON.parse(value.url);

    console.log(imgArray);

    for(i = 0; i < 10; i++) {

        const uddgIndex = urlArray[i].indexOf("uddg=");

        if (uddgIndex !== -1) {
            const uddgPart = urlArray[i].slice(uddgIndex + 5); // "uddg=" sonrasını al, 5 karakteri atla
            const rutIndex = uddgPart.indexOf("&rut=");
            const domain = "undefined";

            if (rutIndex !== -1) {
                const urlWithoutRut = uddgPart.substring(0, rutIndex);   
                const checkUrl = decodeURIComponent(urlWithoutRut);

                const domainMatch = checkUrl.match(/:\/\/([^/]+)(\/.*)/);

                if (domainMatch !== null && domainMatch.length > 2) {
                    const domain = domainMatch[1];
                    const path = domainMatch[2];
                    
                    
                    const resultItem = `
                    <div class="result-item">
                        <div class="result-refs">
                            <div class="result-badge">
                                <img class="result-icon" src="${imgArray[i]}">
                                <b class="result-web font-assistant">${domain}</b>
                            </div>
                            <a href="${decodeURIComponent(urlWithoutRut)}" class="result-href suspend-text font-assistant" style="text-decoration: none">
                                → ${path}
                            </a>
                        </div>
                        <small class="result-box">
                            <div class="result-title font-assistant">${titleArray[i]}</div>
                            <div class="result-desc text-desc font-assistant">
                                ${descArray[i]}
                            </div>
                        </small>
                    </div>
                    `;

                    // Doğru şekilde HTML öğesine dönüştürüp eklemek için aşağıdaki satırı ekledik
                    mainElement.appendChild(htmlToElement(resultItem));
                } else {
                    console.error("Domain parsing error");
                    console.log(urlWithoutRut);
                }
            }
        }
    }
}

function htmlToElement(html) {
    const template = document.createElement('template');
    template.innerHTML = html;
    return template.content.firstElementChild;
}
