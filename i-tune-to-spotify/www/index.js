import * as wasm from "i-tune-to-spotify";

 $(function () {
            $("#uploadBtn").bind("click", function () {
                var regex = /^([a-zA-Z0-9\s_\\.\-:])+(.xml)$/;
                if (regex.test($("#fileUpload").val().toLowerCase())) {
                    if (typeof (FileReader) != "undefined") {
                        var reader = new FileReader();
                        reader.onload = function (e) {
                            wasm.loadlib(reader);
                            alert("Loaded the file.");
                            // e.target.result
                            //TODO display stats on the file
                        }
                        reader.readAsText($("#fileUpload")[0].files[0]);
                    } else {
                        alert("This browser does not support HTML5.");
                    }
                } else {
                    alert("Please upload a valid XML file.");
                }
            });
        });

  $(function () {
            $("#loginBtn").bind("click", function () {
                
                            wasm.spot_login();
                         
            });
        });


  wasm.on_load();