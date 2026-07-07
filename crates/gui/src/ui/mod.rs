slint::slint!(
    export component App inherits Window {
        width: 1200px;
        height: 800px;
        title: "glipho";

        // Header and Main Content
        GridLayout {
            padding: 16px;
            spacing: 8px;

            // Header
            Row {

                Text {
                    text: "Hello glipho!";
                    height: 24px;
                }

            }

            // Main Content
            Row {
                Text {
                    text: "Steps";
                }

                Text {
                    text: "Image preview";
                }
            }
        }

    }
);
