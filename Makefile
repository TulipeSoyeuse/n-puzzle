NAME	= n-puzzle
CARGO	= cargo

all:
	$(CARGO) build --release
	@cp target/release/n-puzzle .

clean:
	$(CARGO) clean

test:
	$(CARGO) test

fclean: clean
	rm -f $(NAME)
	rm -f target/release/$(NAME)

re: fclean all

.PHONY: all clean fclean re
