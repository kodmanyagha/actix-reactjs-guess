import { useEffect } from "react";
import { Col, Row } from "react-bootstrap";
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";
import { useNavigate } from "react-router-dom";
import useApi from "../../../hooks/useApi";
import { setToken, setUser } from "../../../redux/authSlice";
import { useAppDispatch } from "../../../redux/store";

export default function LoginPage() {
  const navigate = useNavigate();
  const api = useApi();
  const dispatch = useAppDispatch();

  useEffect(() => {
    (async () => {
      //
    })();
  }, []);

  async function onFormSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const formJson = Object.fromEntries(new FormData(e.currentTarget).entries());
    const loginResult = await api.post("auth/login", formJson);

    if (loginResult.data.status === "success") {
      dispatch(setToken(loginResult.data.data.token));
      dispatch(setUser(loginResult.data.data.userData));

      navigate("/user");
    } else {
      alert("Please check your credentials.");
    }
  }

  return (
    <>
      <Row className="justify-content-center">
        <Col sm="12" md="12" lg="6">
          <Form onSubmit={onFormSubmit}>
            <Form.Group className="mb-3" controlId="loginForm.email">
              <Form.Label>Email address</Form.Label>
              <Form.Control
                onChange={(e) => {
                  e.target.value;
                }}
                type="email"
                name="email"
                placeholder="name@example.com"
              />
            </Form.Group>
            <Form.Group className="mb-3" controlId="loginForm.password">
              <Form.Label>Password</Form.Label>
              <Form.Control
                type="password"
                name="password"
                placeholder="Password here..."
              />
            </Form.Group>

            <Form.Group className="mb-3" controlId="loginForm.password">
              <Button type="submit" variant="primary" className="w-100 mt-3">
                <i className="fa-solid fa-right-to-bracket" />
                &nbsp; Login
              </Button>
            </Form.Group>
          </Form>
        </Col>
      </Row>
    </>
  );
}
