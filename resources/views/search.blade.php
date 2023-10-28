<!doctype html>
<html lang="{{ str_replace('_', '_', app()->getLocale()) }}">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport"
              content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>Violet Search - {{ $_GET['q'] }}</title>

        <!-- Assets -->
        <link rel="icon" href="{{ asset('image/violet.png') }}">
        <link rel="stylesheet" href="{{ asset('css/app.css') }}">

        <!-- Wiki API -->
        <script src="{{ asset('js/wikipedia.js') }}"></script>

        <!-- Bootstrap Icons -->
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.css">
    </head>
    <body>
        <div id="styled_editor"></div>
        <div class="violet-container is-fluid">
            <div class="violet-row">
                <div class="column-na-12 justify-between">
                    <div class="search-page">
                        <div class="float-end search-page-settings">
                            <div id="sidebar" class="login-button suspend-text font-assistant">Settings</div>
                        </div>
                        <form method="get">
                            <div class="input-group" style="border-radius: 6px;">
                                <img src="{{ asset('image/violet.png') }}" alt="violet-search" />
                                <input id="search-bar" name="q" type="text" value="{{ $_GET['q'] }}" placeholder="What are you looking for?">
                            </div>
                        </form>
                    </div>
                </div>
                <div class="justify-end d-flex responsive-search-page-sidebar">
                    <x-sidebar />
                </div>
            </div>
            <x-modals.themes-and-option />
            <div class="search-types">
                <div class="search-buttons suspend-text font-assistant active">
                    <i class="bi bi-search"></i> &nbsp; All
                </div>
                <div class="search-buttons suspend-text font-assistant">
                    <i class="bi bi-image"></i> &nbsp; Images
                </div>
                <div class="search-buttons suspend-text font-assistant">
                    <i class="bi bi-film"></i> &nbsp; Videos
                </div>
                <div class="search-buttons suspend-text font-assistant">
                    <i class="bi bi-geo-alt"></i> &nbsp; Maps
                </div>
                <div class="search-buttons suspend-text font-assistant">
                    <i class="bi bi-newspaper"></i> &nbsp; News
                </div>
                <div class="search-buttons suspend-text font-assistant" style="color: #9f9fff; cursor: unset;">
                    <i class="bi bi-bounding-box-circles"></i> &nbsp; Results from {{ $_ENV['DEFAULT_SEARCH_ENGINE'] }} (Redirected to Violet Results)
                </div>
            </div>
           <div class="results">
               <div class="violet-row">
                   <div class="column-sm-12 column-xs-12 column-md-8 column-lg-8 column-xl-8">
                       <div class="result-item">
                            <div class="result-refs">
                                <div class="result-badge">
                                    <img class="result-icon" src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.YySVOEBNURN_Q-6h-oETbwHaHE%26pid%3DApi%26h%3D160&f=1&ipt=1e8832aefb2999f1b1ccbbedab868c80df06e71e41503f78671c3501fe34d7b7&ipo=images">
                                    <b class="result-web font-assistant">wiktionary</b>
                                </div>
                                <a class="result-href suspend-text font-assistant">
                                    → merhaba-istanbul
                                </a>
                            </div>
                           <small class="result-box">
                               <div class="result-title font-assistant">merhaba - Wiktionary, the free dictionary</div>
                               <div class="result-desc text-desc font-assistant">
                                   merhaba: hi! ünl. 5: Genel: merhaba: howdy ünl. 6: Genel: merhaba: howdie ünl. 7: Genel: merhaba: hullo ünl. 8: Genel: merhaba: ahey ünl. 9: Genel: merhaba: ciao [italian] ünl. 10: Genel: merhaba: dumela [south africa] exclam. Colloquial: 11: Konuşma Dili: merhaba: ullo [brit] expr. 12: Konuşma Dili: merhaba: hidy expr. 13: Konuşma ...
                               </div>
                           </small>
                       </div>
                   </div>
                   <div class="column-sm-12 column-xs-12 column-md-4 column-lg-4">
                       <div class="suspend-text font-assistant alert-warning">
                           <i class="bi bi-exclamation-triangle"></i> &nbsp; <b>Warning:</b> The Search Engine({{ $_ENV['DEFAULT_SEARCH_ENGINE'] }}) is not working properly. Results redirected to Violet Results.
                       </div>
                       <div id="wikipedia_data"></div>
                       <div id="wiki_titles"></div>
                   </div>
               </div>
           </div>
       </div>
       <script>
           const socket = new WebSocket('ws://127.0.0.1:3000');
           const param = "{{ $_GET['q'] }}";

           socket.onopen = function () {
               socket.send(JSON.stringify({
                   lang: "{{ $_ENV['DEFAULT_LOCALE'] }}",
                   search_engine: "{{ $_ENV['DEFAULT_SEARCH_ENGINE'] }}",
                   search_type: "web",
                   user_agent: "{{ $_SERVER['HTTP_USER_AGENT'] }}",
                   query: param,
                   use_proxy: "{{ $_ENV['USE_VIOLET_PROXY'] }}",
               }));
           };

           socket.onmessage = function (event) {
               console.log(`Data received from server: ${event.data}`);
               const data = JSON.parse(event.data);
           }

           socket.onclose = function (event) {
               if (event.wasClean) {
                   console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
               } else {
                   console.log('[close] Connection died');
               }
           }

           get_wikipedia_titles(param, "{{ $_ENV['DEFAULT_LOCALE'] }}", "wiki_titles");
           get_wikipedia_data(param, "{{ $_ENV['DEFAULT_LOCALE'] }}", "wikipedia_data");
       </script>
       <script src="{{ asset('js/components.js') }}"></script>
    </body>
</html>
