// TODO 判断文件是否存在

function judge(outputFile) {
    try {
        let jsonResult = JSON.parse(outputFile);
        let points = {};
        jsonResult.exercises.forEach(({ name, result }) => {
            if (result) {
                points[name] = [20,20]
            } else {
                points[name] = [0,20]
            }
        })
        return points;
    } catch(e) {
        return {};
    }
}

module.exports.judge = judge;