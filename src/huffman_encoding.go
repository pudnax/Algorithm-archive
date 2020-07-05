package main

import (
	"container/heap"
	"fmt"
)

type Node struct {
	freq  int
	char  rune
	left  *Node
	right *Node
}

type codebook map[rune]string
type nodeHeap []*Node

func (n nodeHeap) Len() int           { return len(n) }
func (n nodeHeap) Less(i, j int) bool { return n[i].freq > n[j].freq }
func (n nodeHeap) Swap(i, j int)      { n[i], n[j] = n[j], n[i] }

func (n *nodeHeap) Push(x interface{}) {
	if node, ok := x.(*Node); ok {
		*n = append(*n, node)
	} else {
		fmt.Printf("I got a node of type %T.\n", x)
	}
}

func (n *nodeHeap) Pop() interface{} {
	old := *n
	l := len(old)
	x := old[l-1]
	*n = old[0 : l-1]
	return x
}

func buildTree(message string) *Node {
	freqMap := make(map[rune]*Node)
	h := new(nodeHeap)
	heap.Init(h)

	for _, char := range message {
		if _, ok := freqMap[char]; ok {
			freqMap[char].freq += 1
		} else {
			newNode := new(Node)
			newNode.freq = 1
			newNode.char = char
			freqMap[char] = newNode
			heap.Push(h, newNode)
		}
	}

	for h.Len() > 1 {
		left, right := h.Pop().(*Node), h.Pop().(*Node)
		branch := new(Node)
		branch.freq = right.freq + left.freq
		branch.left = left
		branch.right = right
		heap.Push(h, branch)
	}

	root := heap.Pop(h).(*Node)
	return root
}

func generateCodebook(root Node) codebook {
	cb := make(codebook)
	codeArr := make([]rune, 0)
	codebookRecurse(root, &cb, codeArr)
	return cb
}

func codebookRecurse(node *Node, cb *codebook, code []rune) {
	if node == nil {
		return
	}

	if node.left == nil && node.right == nil {
		(*cb)[node.char] = string(code)
	}

	code = append(code, '0')
	codebookRecurse(node.left, cb, code)
	code = append(code[:len(code)-1], '1')
	codebookRecurse(node.right, cb, code)
}

func encode(message string) (string, *Node, codebook) {
	ret := ""
	root := buildTree(message)
	cb := generateCodebook(root)
	for _, char := range message {
		ret += cb[char]
	}

	return ret, root, cb
}

func decode(message string, root *Node) string {
	cur := root
	ret := ""

	for _, char := range message {
		if cur == nil {
			return message
		}

		switch string(char) {
		case "0":
			if cur.left == nil {
				ret += string(cur.char)
				cur = root.left
			} else {
				cur = cur.left
			}
		case "1":
			if cur.right == nil {
				ret += string(cur.char)
				cur = root.right
			} else {
				cur = cur.right
			}
		}
	}

	if cur.char != 0 {
		ret += string(cur.char)
	}

	return ret
}

func main() {
	enc, root, cb := encode("bibbity_bobbity")
	fmt.Println("Codebook:")
	for r, c := range cb {
		fmt.Println(string(r), "->", c)
	}
	fmt.Println("\nEncoded:", enc)
	fmt.Println("Deocded:", decode(enc, root))
}
