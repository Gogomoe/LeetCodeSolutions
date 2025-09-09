package main

type LRUNode struct {
	Key  int
	Val  int
	Next *LRUNode
	Pre  *LRUNode
}

type LRUCache struct {
	Head     *LRUNode
	Tail     *LRUNode
	Map      map[int]*LRUNode
	Capacity int
	Size     int
}

func NewLRUCache(capacity int) LRUCache {
	head := &LRUNode{}
	tail := &LRUNode{}
	head.Next = tail
	tail.Pre = head
	return LRUCache{
		Head:     head,
		Tail:     tail,
		Map:      make(map[int]*LRUNode),
		Capacity: capacity,
		Size:     0,
	}
}

func (this *LRUCache) Get(key int) int {
	item, ok := this.Map[key]
	if !ok {
		return -1
	}
	this.removeItemFromChain(item)
	this.insertToFirst(item)
	return item.Val
}

func (this *LRUCache) Put(key int, value int) {
	if this.Get(key) != -1 {
		first := this.Head.Next
		first.Val = value
		return
	}
	item := &LRUNode{
		Key: key,
		Val: value,
	}
	this.insertToFirst(item)
	this.Map[key] = item
	if this.Size < this.Capacity {
		this.Size++
		return
	}
	last := this.Tail.Pre
	this.removeItemFromChain(last)
	delete(this.Map, last.Key)
}

func (this *LRUCache) insertToFirst(item *LRUNode) {
	head := this.Head
	first := head.Next

	head.Next = item
	item.Pre = head
	item.Next = first
	first.Pre = item
}

func (this *LRUCache) removeItemFromChain(item *LRUNode) {
	pre := item.Pre
	next := item.Next

	pre.Next = next
	next.Pre = pre
}

func main() {
	lru := NewLRUCache(2)
	lru.Put(1, 1)
	lru.Put(2, 2)
	println(lru.Get(1))
	lru.Put(3, 3)
	println(lru.Get(2))
}
