class Nodex {
    val: boolean
    isLeaf: boolean
    topLeft: Nodex | null
    topRight: Nodex | null
    bottomLeft: Nodex | null
    bottomRight: Nodex | null
    constructor(val?: boolean, isLeaf?: boolean, topLeft?: Nodex, topRight?: Nodex, bottomLeft?: Nodex, bottomRight?: Nodex) {
        this.val = (val === undefined ? false : val)
        this.isLeaf = (isLeaf === undefined ? false : isLeaf)
        this.topLeft = (topLeft === undefined ? null : topLeft)
        this.topRight = (topRight === undefined ? null : topRight)
        this.bottomLeft = (bottomLeft === undefined ? null : bottomLeft)
        this.bottomRight = (bottomRight === undefined ? null : bottomRight)
    }
}

let grid: number[][] = [
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0]
];

function construct(grid: number[][]): Nodex | null {
    return split(0, grid[0].length, 0, grid.length, grid);
};

function split(left: number, right: number, top: number, bottom: number, grid: number[][]): Nodex | null {
    let node: Nodex = new Nodex;
    let match: number | null = null;
    let divide: boolean = false;

    for (let y = top; y < bottom; y++) {
        for (let x = left; x < right; x++) {
            if (match == null) {
                match = grid[y][x];
            } else if (grid[y][x] != match) {
                divide = true;
                break;
            }
        }

        if (divide) {
            break;
        }
    }

    if (divide) {
        let horMid = (left + right) / 2;
        let verMid = (top + bottom) / 2;

        node.isLeaf = false;
        node.val = true;
        node.topLeft = split(left, horMid, top, verMid, grid);
        node.bottomLeft = split(left, horMid, verMid, bottom, grid);
        node.topRight = split(horMid, right, top, verMid, grid);
        node.bottomRight = split(horMid, right, verMid, bottom, grid);
    } else {
        node.isLeaf = true;
        node.val = match == 0 ? false : true;
    }

    return node;
}