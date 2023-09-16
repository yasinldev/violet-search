<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Http;

/*

    This controller is responsible for handling the search page.
    It will be used to display the search page and handle the search request.

    Checking if the search query is set and not empty, if it is, redirect to the home page. LN: 19-21.
    If the search query is set and not empty, return the search view. LN: 23.

    The fetchData method is responsible for fetching the data from the Rust microservice.
    It will send an HTTP request to the Rust microservice and return the response as a JSON response.
    The fetchData method will be called by the JavaScript code in the search view. LN: 25-34.

*/

class SettingsController extends Controller {

    public function index(): \Illuminate\Foundation\Application|\Illuminate\Contracts\View\View|\Illuminate\Contracts\View\Factory|\Illuminate\Routing\Redirector|\Illuminate\Contracts\Foundation\Application|\Illuminate\Http\RedirectResponse {
        return view('settings');
    }
}
