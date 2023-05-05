import React, { useEffect }  from "react";
import ReactDOM from "react-dom";
// import "./index.scss";

const wasm = import("../build/rgantt");

wasm.then((m) => {
    const App = () => {
        const tasks_js = `[
            {
                "start": "2023-04-20 01:46:39",
                "end": "2023-06-20 01:46:39",
                "name": "Some Project",
                "id": "ProjectSample",
                "progress": 25,
                "type": "project",
                "hideChildren": false,
                "displayOrder": 1
            },
            {
                "start": "2023-04-01 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Idea",
                "id": "Task 0",
                "progress": 45,
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 2
            },
            {
                "start": "2023-04-20 01:46:39",
                "end": "2023-05-20 01:46:39",
                "name": "Research",
                "id": "Task 1",
                "progress": 25,
                "dependencies": [
                    "Task 0"
                ],
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 3
            },
            {
                "start": "2023-04-01 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Discussion with team",
                "id": "Task 2",
                "progress": 10,
                "dependencies": [
                    "Task 1"
                ],
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 4
            },
            {
                "start": "2023-04-13 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Developing",
                "id": "Task 3",
                "progress": 2,
                "dependencies": [
                    "Task 2"
                ],
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 5
            },
            {
                "start": "2023-04-05 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Review",
                "id": "Task 4",
                "type": "task",
                "progress": 70,
                "dependencies": [
                    "Task 2"
                ],
                "project": "ProjectSample",
                "displayOrder": 6
            },
            {
                "start": "2023-04-11 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Release",
                "id": "Task 6",
                "progress": 40,
                "type": "milestone",
                "dependencies": [
                    "Task 4"
                ],
                "project": "ProjectSample",
                "displayOrder": 7
            },
            {
                "start": "2023-05-10 01:46:39",
                "end": "2023-05-20 01:46:39",
                "name": "Party Time",
                "id": "Task 9",
                "progress": 0,
                "isDisabled": true,
                "type": "task"
            }
        ]`;
        useEffect(() => {
            m.render("#gantt_id", tasks_js, "", "", "");
        }, [])

        return (
            <div>
                <h1>Hello, Gannt !</h1>
                <div id={"gantt_id"} />
            </div>
        );
    };

    ReactDOM.render(<App></App>, document.getElementById("root"));
});
