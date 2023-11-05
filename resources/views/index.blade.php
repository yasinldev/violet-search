<!doctype html>
<?php session_start(); ?>
<html lang="{{ str_replace('_', '_', app()->getLocale()) }}">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport"
              content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>Violet Search</title>

        <!-- Assets -->
        <link rel="icon" href="{{ asset('image/violet.png') }}">
        <link rel="stylesheet" href="{{ asset('css/app.css') }}">

        <!-- Bootstrap Icons -->
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.10.5/font/bootstrap-icons.css">
    </head>
    <body id="index-body">
    <div id="styled_editor"></div>
    <div class="violet-container is-fluid">
        <div class="violet-row">
            <div class="column-na-12">
                <div class="d-flex justify-between items-center">
                    <img src="{{ asset('image/violet.png') }}" alt="violet-search" style="max-width: 60px;" />
                    <div class="d-flex">
                        <div id="sidebar" class="login-button suspend-text font-assistant">Settings</div>
                    </div>
                </div>
                <div class="justify-end d-flex">
                    <x-sidebar />
                </div>
            </div>
        </div>
        <x-modals.themes-and-option />
        <form action="{{ route('search') }}" method="get">
            <div class="violet-row">
                <div class="column-na-12 justify-center items-center">
                    <div class="input-group responsive-margin-top">
                        <i class="bi bi-search icon-color"></i>
                        <input id="search-bar" name="q" type="text" placeholder="What are you looking for?">
                    </div>
                    <script>
                        // Creating a websocket
                        const socket = new WebSocket('ws://127.0.0.1:3000');
                        const search_bar = document.getElementById('search-bar');
                        const search_engine = "{{ $_ENV['DEFAULT_SEARCH_ENGINE'] }}";

                        // When the connection is open, send some data to the server
                        socket.onopen = function () {
                            // Sending a Json value for connection
                            socket.send(JSON.stringify({
                                lang: "{{ $_ENV['DEFAULT_LOCALE'] }}",
                                search_engine: "{{ $_ENV['DEFAULT_SEARCH_ENGINE'] }}",
                                search_type: "dropdown",
                                user_agent: "{{ $_SERVER['HTTP_USER_AGENT'] }}",
                                query: null,
                                use_proxy: "{{ $_ENV['USE_VIOLET_PROXY'] }}",
                            }));
                        };

                        search_bar.addEventListener('input', function() {
                            const inputValue = search_bar.value.trim();
                            const removable = document.getElementById('removable');

                            if (inputValue.length >= 3) {
                                socket.send(JSON.stringify({
                                    lang: "{{ $_ENV['DEFAULT_LOCALE'] }}",
                                    search_engine: "{{ $_ENV['DEFAULT_SEARCH_ENGINE'] }}",
                                    search_type: "dropdown",
                                    user_agent: "{{ $_SERVER['HTTP_USER_AGENT'] }}",
                                    query: inputValue,
                                    use_proxy: "{{ $_ENV['USE_VIOLET_PROXY'] }}",
                                }))

                            }
                        });

                        socket.onmessage = function (event) {
                            // Parsing the Json value
                            const data = JSON.parse(event.data);
                            const dropdown = document.getElementById('suggestion');
                            dropdown.innerHTML = '';

                            if (search_engine === "google") {
                                scrap_google_suggestions(data)
                            }
                            else if (search_engine === "duckduckgo") {
                                scrap_ddg_suggestions(data)
                            }
                            else if (search_engine === "artado") {
                                scrap_artado_suggestions(data)
                            }
                            else {
                                console.alert("Warning: Search Engine is not working properly. Results redirected to Violet Results.")
                            }
                        };

                        socket.onclose = function (event) {
                            if (event.wasClean) {
                                console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
                            } else {
                                console.log('[close] Connection died');
                            }
                        }
                    </script>
                    <x-Input-dropdown />
                    
                </div>
            </div>
        </form>
    </div>
    <script src="{{ asset('js/parser/dropdowns.js') }}"></script>
    <script src="{{ asset('js/components.js') }}"></script>
    </body>
</html>
