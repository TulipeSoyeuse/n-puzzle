NAME	= n-puzzle
CARGO	= cargo

all:
	$(CARGO) install --path .

clean:
	$(CARGO) clean

test:
	$(CARGO) test

fclean: clean
	$(CARGO) uninstall

re: fclean all

.PHONY: all clean fclean re
