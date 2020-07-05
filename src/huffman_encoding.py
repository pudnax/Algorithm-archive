from collections import Counter


def build_huffman_tree(message):
    frequencies = Counter(message)
    trees = frequencies.most_common()

    while len(trees) > 1:
        tree_left, weight_left = trees.pop()
        tree_right, weight_right = trees.pop()

        new_tree = [tree_left, tree_right]
        new_weight = weight_left + weight_right

        index = next(
            (i for i, tree in enumerate(trees) if tree[1] < new_weight),
            len(trees))

        trees.insert(index, (new_tree, new_weight))

    huffman_tree = trees[0][0]
    return huffman_tree


def build_codebook(tree, code=''):
    codebook = []

    left_tree, right_tree = tree

    if type(left_tree) is list:
        codebook += build_codebook(left_tree, code + '0')
    else:
        codebook.append((left_tree, code + '0'))

    if type(right_tree) is list:
        codebook += build_codebook(right_tree, code + '1')
    else:
        codebook.append((right_tree, code + '1'))

    return codebook


def huffman_encode(codebook, message):
    encoded_message = ''

    forward_dict = dict(codebook)

    for char in message:
        encoded_message += forward_dict[char]

    return encoded_message


def huffman_decode(codebook, encoded_message):

    decoded_message = ''
    key = ''

    inverse_dict = dict([(v, k) for k, v in codebook])

    for _, bit in enumerate(encoded_message):
        key += bit
        if key in inverse_dict:
            decoded_message += inverse_dict[key]
            key = ''

    return decoded_message


def main():

    message = 'bibbity_bobbity'
    tree = build_huffman_tree(message)
    codebook = build_codebook(tree)
    encoded_message = huffman_encode(codebook, message)
    decoded_message = huffman_decode(codebook, encoded_message)

    print('message: ' + message)
    print('huffman tree: ' + str(tree))
    print('codebook: ' + str(codebook))
    print('encoded message: ' + encoded_message)
    print('decoded message: ' + decoded_message)


if __name__ == '__main__':
    main()
